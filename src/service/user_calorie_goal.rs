use crate::{
    model::{input::SetCalorieGoalInput, output::UserCalorieGoalOutput},
    service::error::ServiceError,
    source,
};
use chrono::{Duration, NaiveDate};

pub async fn get_current_calorie_goal(
    user_id: i32,
) -> Result<Option<UserCalorieGoalOutput>, ServiceError> {
    let goal = source::user_calorie_goal::get_current_calorie_goal(user_id).await?;
    Ok(goal.map(|g| g.into()))
}

pub async fn set_calorie_goal(
    user_id: i32,
    input: SetCalorieGoalInput,
) -> Result<UserCalorieGoalOutput, ServiceError> {
    let effective_from = NaiveDate::parse_from_str(&input.effective_from, "%Y-%m-%d")
        .map_err(|_| ServiceError::InvalidInput("invalid effective_from date format".into()))?;

    let today = chrono::Local::now().date_naive();

    // If effective_from is today, update current goal
    if effective_from == today {
        if let Some(current) = source::user_calorie_goal::get_current_calorie_goal(user_id).await? {
            let goal = source::user_calorie_goal::update_calorie_goal(
                current.id,
                input.daily_calorie_goal,
                None,
            )
            .await?;
            return Ok(goal.into());
        }
    }

    // Close current goal if exists
    if let Some(current) = source::user_calorie_goal::get_current_calorie_goal(user_id).await? {
        let close_date = effective_from - Duration::days(1);
        source::user_calorie_goal::update_calorie_goal(
            current.id,
            current.daily_calorie_goal,
            Some(close_date),
        )
        .await?;
    }

    let goal =
        source::user_calorie_goal::create_calorie_goal(user_id, input.daily_calorie_goal, effective_from)
            .await?;
    Ok(goal.into())
}