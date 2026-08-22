use crate::dto::UpdateExpenseRequest;
use crate::{dto::ExpenseRequest, model::Expense};
use chrono::{Datelike, Utc};
use std::path::Path;
use std::{error::Error, fs};

// Load expenses
pub fn load_expenses(file_name: &str) -> Result<Vec<Expense>, Box<dyn Error>> {
    if !Path::new(file_name).exists() {
        return Ok(Vec::new());
    }

    let data = fs::read_to_string(file_name)?;

    if data.trim().is_empty() {
        return Ok(Vec::new());
    }

    let expenses = serde_json::from_str(&data)?;

    Ok(expenses)
}

// Save Expense
fn save_expense(file_name: &str, expenses: &[Expense]) -> Result<(), Box<dyn Error>> {
    let data = serde_json::to_string_pretty(expenses)?;
    fs::write(file_name, data)?;

    Ok(())
}

// Create Expense
pub fn create_expense(
    file_name: &str,
    expense_request: ExpenseRequest,
) -> Result<(), Box<dyn Error>> {
    let mut expenses = load_expenses(file_name)?;

    let timing = Utc::now();

    let expense = Expense {
        amount: expense_request.amount,
        id: expenses.iter().map(|e| e.id).max().unwrap_or(0) + 1,
        categories: expense_request.category,
        description: expense_request.description,
        created_at: timing,
        updated_at: timing,
        month: timing.month(),
    };

    expenses.push(expense);

    save_expense(file_name, &expenses)?;

    Ok(())
}

// Read Particular Expense (by ID)
pub fn get_specific_expense(
    expense_id: u32,
    file_name: &str,
) -> Result<Option<Expense>, Box<dyn Error>> {
    let expenses = load_expenses(file_name)?;
    Ok(expenses.into_iter().find(|x| x.id == expense_id))
}

// Update Expense
pub fn update_expense(
    expense_id: u32,
    file_name: &str,
    expense_request: UpdateExpenseRequest,
) -> Result<(), Box<dyn Error>> {
    let mut expenses = load_expenses(file_name)?;

    let expense = expenses
        .iter_mut()
        .find(|expense| expense.id == expense_id)
        .ok_or("Expense not found")?;

    if let Some(description) = expense_request.description {
        expense.description = description;
    }

    if let Some(amount) = expense_request.amount {
        expense.amount = amount;
    }

    if let Some(category) = expense_request.category {
        expense.categories = category;
    }

    expense.updated_at = Utc::now();

    save_expense(file_name, &expenses)?;

    Ok(())
}

// Delete Expense
pub fn delete_expense(expense_id: u32, file_name: &str) -> Result<(), Box<dyn Error>> {
    let mut expenses = load_expenses(file_name)?;

    let original_len = expenses.len();

    expenses.retain(|expense| expense.id != expense_id);
    if expenses.len() == original_len {
        return Err("Expense not found".into());
    }
    save_expense(file_name, &expenses)
}

// Get Summary of Expense
pub fn get_summary_of_expense(file_name: &str) -> Result<u32, Box<dyn Error>> {
    let expenses = load_expenses(file_name)?;

    let total: u32 = expenses.iter().map(|expense| expense.amount).sum();

    Ok(total)
}

pub fn list_expenses(file_name: &str) -> Result<(), Box<dyn Error>> {
    let expenses = load_expenses(file_name)?;

    if expenses.is_empty() {
        println!("No expenses found.");
        return Ok(());
    }

    for expense in &expenses {
        println!(
            "#{} | {} | {} | {:?} | {}",
            expense.id, expense.description, expense.amount, expense.categories, expense.updated_at
        );
    }
    Ok(())
}
