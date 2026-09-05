use crate::{
    core::error::AppError,
    dto::{ExpenseRequest, UpdateExpenseRequest},
    model::Expense,
    repository,
};
use sqlx::PgPool;

pub async fn create_expense(
    pool: &PgPool,
    expense_request: ExpenseRequest,
) -> Result<Expense, AppError> {
    let expense = repository::create_expense(pool, expense_request).await?;
    Ok(expense)
}

pub async fn list_expenses(pool: &PgPool) -> Result<Vec<Expense>, AppError> {
    let expenses = repository::list_expenses(pool).await?;
    Ok(expenses)
}

pub async fn find_expense_by_id(pool: &PgPool, expense_id: i32) -> Result<Expense, AppError> {
    let expense = repository::find_expense_by_id(pool, expense_id).await?;
    Ok(expense)
}

pub async fn update_expense(
    pool: &PgPool,
    expense_id: i32,
    expense_request: UpdateExpenseRequest,
) -> Result<Expense, AppError> {
    let expense = repository::update_expense(pool, expense_id, expense_request).await?;
    Ok(expense)
}

pub async fn delete_expense(pool: &PgPool, expense_id: i32) -> Result<(), AppError> {
    repository::delete_expense(pool, expense_id).await?;
    Ok(())
}