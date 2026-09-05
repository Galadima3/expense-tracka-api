use crate::{
    core::{app_state::AppState, error::AppError},
    dto::{ExpenseRequest, UpdateExpenseRequest},
    service::{create_expense, delete_expense, find_expense_by_id, list_expenses, update_expense},
};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use validator::Validate;

pub async fn create_expense_handler(
    State(state): State<AppState>,
    Json(payload): Json<ExpenseRequest>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate()?;

    let expense = create_expense(&state.db_pool, payload).await?;
    Ok((StatusCode::CREATED, Json(expense)))
}

pub async fn list_expenses_handler(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let expenses = list_expenses(&state.db_pool).await?;
    Ok(Json(expenses))
}

pub async fn update_expense_handler(
    State(state): State<AppState>,
    Path(expense_id): Path<i32>,
    Json(payload): Json<UpdateExpenseRequest>,
) -> Result<impl IntoResponse, AppError> {
    let expense = update_expense(&state.db_pool, expense_id, payload).await?;
    Ok(Json(expense))
}

pub async fn delete_expense_handler(
    State(state): State<AppState>,
    Path(expense_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    delete_expense(&state.db_pool, expense_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn find_expense_handler(
    State(state): State<AppState>,
    Path(expense_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let expense = find_expense_by_id(&state.db_pool, expense_id).await?;
    Ok((StatusCode::OK, Json(expense)))
}