use crate::{db, model::raw::UserCalorieGoal};

pub async fn get_current_calorie_goal(user_id: i32) -> Result<Option<UserCalorieGoal>, sqlx::Error> {
    let goal: Option<UserCalorieGoal> = sqlx::query_as(
        r#"
        SELECT id, user_id, daily_calorie_goal, effective_from, effective_to, created_at, updated_at
        FROM user_calorie_goal
        WHERE user_id = $1
          AND effective_from <= CURRENT_DATE
          AND (effective_to IS NULL OR effective_to >= CURRENT_DATE)
        ORDER BY effective_from DESC
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_optional(db())
    .await?;
    Ok(goal)
}

pub async fn create_calorie_goal(
    user_id: i32,
    daily_calorie_goal: f32,
    effective_from: chrono::NaiveDate,
) -> Result<UserCalorieGoal, sqlx::Error> {
    let goal: UserCalorieGoal = sqlx::query_as(
        r#"
        INSERT INTO user_calorie_goal (user_id, daily_calorie_goal, effective_from)
        VALUES ($1, $2, $3)
        RETURNING id, user_id, daily_calorie_goal, effective_from, effective_to, created_at, updated_at
        "#,
    )
    .bind(user_id)
    .bind(daily_calorie_goal)
    .bind(effective_from)
    .fetch_one(db())
    .await?;
    Ok(goal)
}

pub async fn update_calorie_goal(
    id: i32,
    daily_calorie_goal: f32,
    effective_to: Option<chrono::NaiveDate>,
) -> Result<UserCalorieGoal, sqlx::Error> {
    let goal: UserCalorieGoal = sqlx::query_as(
        r#"
        UPDATE user_calorie_goal
        SET daily_calorie_goal = $1, effective_to = $2
        WHERE id = $3
        RETURNING id, user_id, daily_calorie_goal, effective_from, effective_to, created_at, updated_at
        "#,
    )
    .bind(daily_calorie_goal)
    .bind(effective_to)
    .bind(id)
    .fetch_one(db())
    .await?;
    Ok(goal)
}