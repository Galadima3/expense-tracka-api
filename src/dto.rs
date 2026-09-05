use crate::model::Categories;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct ExpenseRequest {
    #[validate(length(min = 1, message = "Description cannot be empty"))]
    pub description: String,

    #[validate(range(min = 1, message = "Amount must be greater than 0"))]
    pub amount: i32,

    pub category: Categories,
}

#[derive(Debug, Validate, Deserialize)]
pub struct UpdateExpenseRequest {
    #[validate(length(min = 1, message = "Description cannot be empty"))]
    pub description: Option<String>,

    #[validate(range(min = 1, message = "Amount must be greater than 0"))]
    pub amount: Option<i32>,

    pub category: Option<Categories>,
}