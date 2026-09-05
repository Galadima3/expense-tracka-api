use chrono::{Datelike, Utc};
use sqlx::PgPool;

use crate::dto::{ExpenseRequest, UpdateExpenseRequest};
use crate::model::Expense;

pub async fn create_expense(
    pool: &PgPool,
    expense_request: ExpenseRequest,
) -> Result<Expense, sqlx::Error> {
    let month = Utc::now().month() as i32;

    let new_expense = sqlx::query_as::<_, Expense>(
        r#"
        INSERT INTO expenses (description, amount, month, category)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(&expense_request.description)
    .bind(expense_request.amount)
    .bind(month)
    .bind(&expense_request.category)
    .fetch_one(pool)
    .await?;

    Ok(new_expense)
}

pub async fn list_expenses(pool: &PgPool) -> Result<Vec<Expense>, sqlx::Error> {
    sqlx::query_as::<_, Expense>("SELECT * FROM expenses ORDER BY created_at DESC")
        .fetch_all(pool)
        .await
}

pub async fn find_expense_by_id(pool: &PgPool, expense_id: i32) -> Result<Expense, sqlx::Error> {
    sqlx::query_as::<_, Expense>(
        r#"
        SELECT * FROM expenses WHERE id = $1
        "#,
    )
    .bind(expense_id)
    .fetch_one(pool)
    .await
}

pub async fn update_expense(
    pool: &PgPool,
    expense_id: i32,
    expense_request: UpdateExpenseRequest,
) -> Result<Expense, sqlx::Error> {
    sqlx::query_as::<_, Expense>(
        r#"
        UPDATE expenses
        SET description = COALESCE($1, description),
            amount = COALESCE($2, amount),
            category = COALESCE($3, category),
            updated_at = NOW()
        WHERE id = $4
        RETURNING *
        "#,
    )
    .bind(&expense_request.description)
    .bind(expense_request.amount)
    .bind(expense_request.category)
    .bind(expense_id)
    .fetch_one(pool)
    .await
}

pub async fn delete_expense(pool: &PgPool, expense_id: i32) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM expenses WHERE id = $1")
        .bind(expense_id)
        .execute(pool)
        .await?;

    Ok(())
}