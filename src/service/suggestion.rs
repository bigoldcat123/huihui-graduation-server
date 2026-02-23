use crate::{
    model::{input::{AddTodoLogInput, CreateSuggestionInput, MoveSuggestionNextInput, ReviewSuggestionInput}, output::{FoodTag, FoodWithTags, Restaurant, Suggestion, TodoLogItem}},
    service::error::ServiceError,
    source,
};

pub async fn create(user_id: i32, ipt: CreateSuggestionInput) -> Result<i32, ServiceError> {
    let images_json = serde_json::to_string(&ipt.images)?;
    let new_id = source::suggestion::create_suggestion(
        user_id,
        &ipt.content,
        &images_json,
        ipt.r#type.as_db_str(),
        ipt.food_id,
        ipt.restaurant_id,
    )
    .await?;
    Ok(new_id)
}

pub async fn list_my(user_id: i32) -> Result<Vec<Suggestion>, ServiceError> {
    let suggestions = source::suggestion::list_my_suggestions(user_id).await?;
    map_suggestions(suggestions).await
}

pub async fn list_by_page(
    page: Option<i64>,
    page_size: Option<i64>,
    status: Option<String>,
    suggestion_type: Option<String>,
) -> Result<Vec<Suggestion>, ServiceError> {
    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(10);
    let status = normalize_filter_enum(status);
    let suggestion_type = normalize_filter_enum(suggestion_type);
    log::info!("status: {:?}, type: {:?}", status, suggestion_type);
    let suggestions = source::suggestion::list_suggestions_by_page(
        page,
        page_size,
        status.as_deref(),
        suggestion_type.as_deref(),
    )
    .await?;
    map_suggestions(suggestions).await
}

pub async fn get_by_id(suggestion_id: i32) -> Result<Suggestion, ServiceError> {
    let suggestion = source::suggestion::get_suggestion_by_id(suggestion_id).await?;
    let mut data = map_suggestions(vec![suggestion]).await?;
    data.pop().ok_or(ServiceError::SqlError(sqlx::Error::RowNotFound))
}

pub async fn list_todos_by_page(page: Option<i64>, page_size: Option<i64>) -> Result<Vec<Suggestion>, ServiceError> {
    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(10);
    let suggestions = source::suggestion::list_todos_by_page(page, page_size).await?;
    map_suggestions(suggestions).await
}

pub async fn review(root_user_id: i32, ipt: ReviewSuggestionInput) -> Result<(), ServiceError> {
    source::suggestion::review_suggestion(
        ipt.suggestion_id,
        root_user_id,
        ipt.status.as_db_str(),
        &ipt.review_comment,
    )
    .await?;
    Ok(())
}

pub async fn move_to_next_stage(ipt: MoveSuggestionNextInput) -> Result<String, ServiceError> {
    let suggestion = source::suggestion::get_suggestion_by_id(ipt.suggestion_id).await?;
    let current = suggestion.status.to_uppercase();
    let next = match current.as_str() {
        "APPROVED" => "PREPARING",
        "PREPARING" => "PROCESSING",
        "PROCESSING" => "FINISHED",
        "FINISHED" => {
            return Err(ServiceError::PermissionDenied(
                "suggestion already at final stage FINISHED".to_string(),
            ));
        }
        _ => {
            return Err(ServiceError::PermissionDenied(format!(
                "status {} cannot move to next stage",
                suggestion.status
            )));
        }
    };

    source::suggestion::update_status_if_current(ipt.suggestion_id, &current, next).await?;
    let _ = source::todo_log::create_todo_log(
        ipt.suggestion_id,
        next,
        &format!("move suggestion {} from {} to {}", ipt.suggestion_id, current, next),
    )
    .await?;
    Ok(next.to_string())
}

pub async fn add_todo_log_by_current_status(ipt: AddTodoLogInput) -> Result<i32, ServiceError> {
    let expected = if ipt.current_status.trim().eq_ignore_ascii_case("ACCEPTED") {
        "APPROVED".to_string()
    } else {
        ipt.current_status.trim().to_uppercase()
    };
    let suggestion = source::suggestion::get_suggestion_by_id(ipt.suggestion_id).await?;
    let current = suggestion.status.to_uppercase();
    if current != expected {
        return Err(ServiceError::PermissionDenied(format!(
            "status mismatch, current is {}, input is {}",
            current, expected
        )));
    }
    let new_id = source::todo_log::create_todo_log(
        ipt.suggestion_id,
        &current,
        &ipt.log_content,
    )
    .await?;
    Ok(new_id)
}

pub async fn list_todo_logs(
    suggestion_id: i32,
    suggestion_status: String,
) -> Result<Vec<TodoLogItem>, ServiceError> {
    let db_status = suggestion_status.trim().to_uppercase();

    if db_status == "APPROVED" {
        let suggestion = source::suggestion::get_suggestion_by_id(suggestion_id).await?;
        return Ok(vec![TodoLogItem {
            content: suggestion.content,
            create_time: suggestion.created_at.format("%Y-%m-%d").to_string(),
        }]);
    }

    let logs = source::todo_log::list_todo_logs_by_suggestion_and_status(
        suggestion_id,
        &db_status,
    )
    .await?;

    Ok(logs
        .into_iter()
        .map(|x| TodoLogItem {
            content: x.content,
            create_time: x.create_time.format("%Y-%m-%d").to_string(),
        })
        .collect())
}

async fn map_suggestions(suggestions: Vec<crate::model::raw::Suggestion>) -> Result<Vec<Suggestion>, ServiceError> {
    let mut result = Vec::with_capacity(suggestions.len());

    for item in suggestions {
        let food = match item.food_id {
            Some(food_id) => {
                let food = source::food::get_food_by_id(food_id).await?;
                let restaurant = source::restaurant::get_restaurant_by_id(food.restaurant_id).await?;
                let tags = source::tag::list_food_tags(food.id).await?;
                Some(FoodWithTags {
                    id: food.id,
                    restaurant_id: food.restaurant_id,
                    restaurant_name: restaurant.name,
                    name: food.name,
                    description: food.description,
                    image: food.image,
                    tags: tags
                        .into_iter()
                        .map(|t| FoodTag {
                            id: t.id,
                            name: t.name,
                            image: t.image,
                        })
                        .collect(),
                })
            }
            None => None,
        };

        let restaurant = match item.restaurant_id {
            Some(restaurant_id) => {
                let r = source::restaurant::get_restaurant_by_id(restaurant_id).await?;
                Some(Restaurant::from(r))
            }
            None => None,
        };

        result.push(Suggestion {
            id: item.id,
            content: item.content,
            images: item
                .images
                .and_then(|images| serde_json::from_str::<Vec<String>>(&images).ok()),
            r#type: item.r#type,
            status: item.status,
            food,
            restaurant,
            reviewer_id: item.reviewer_id,
            review_comment: item.review_comment,
            user_id: item.user_id,
            created_at: item.created_at.format("%Y-%m-%d").to_string(),
            reviewed_at: item
                .reviewed_at
                .map(|t| t.format("%Y-%m-%d").to_string()),
        });
    }

    Ok(result)
}

fn normalize_filter_enum(input: Option<String>) -> Option<String> {
    input
        .map(|s| s.trim().to_uppercase())
        .filter(|s| !s.is_empty())
}
