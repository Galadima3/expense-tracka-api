use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand, ValueEnum};
use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Expense {
    pub id: u32,
    pub description: String,
    pub amount: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub month: u32,
    pub categories: Categories,
}

#[derive(Debug)]
pub struct ExpenseRequest {
    pub description: String,
    pub amount: u32,
    pub category: Categories,
}

pub struct UpdateExpenseRequest {
    pub description: Option<String>,
    pub amount: Option<u32>,
    pub category: Option<Categories>,
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
impl fmt::Display for Categories {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Categories::Food => "Food",
            Categories::Transport => "Transport",
            Categories::Housing => "Housing",
            Categories::Utilities => "Utilities",
            Categories::Entertainment => "Entertainment",
        };
        write!(f, "{}", value)
    }
}

#[derive(Parser, Debug)]
#[command(name = "expense-tracker")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Add {
        #[arg(short, long)]
        description: String,
        #[arg(short, long)]
        amount: u32,
        #[arg(short, long)]
        category: Categories,
    },
    // Fetch Expense
    Fetch {
        #[arg(short, long)]
        id: u32,
    },
    // Update Expense
    Update {
        #[arg(short, long)]
        id: u32,
        #[arg(short, long)]
        description: Option<String>,

        #[arg(short, long)]
        amount: Option<u32>,

        #[arg(short, long)]
        category: Option<Categories>,
    },

    Delete {
        #[arg(short, long)]
        id: u32,
    },

    List,

    Summary,

    Quit,
}
