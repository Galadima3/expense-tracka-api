use core::fmt;
use clap::ValueEnum;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Expense {
    pub id: u32,
    pub description: String,
    pub amount: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub month: u32,
    pub categories: Categories
}


#[derive(Debug, ValueEnum, Clone, Copy, Deserialize, Serialize)]
#[value(rename_all = "lower")]
pub enum Categories {
    Food,
    Transport,
    Housing,
    Utilities,
    Entertainment,
}
impl fmt::Display for Categories{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Categories::Food => "Food",
            Categories::Transport => "Transport",
            Categories::Housing => "Housing",
            Categories::Utilities => "Utilities",
            Categories::Entertainment => "Entertainment"
        };
        write!(f, "{}", value)
    }
}