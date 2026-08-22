//Expense Request
use crate::model::Categories;
use clap::{Parser, Subcommand};

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
    // Delete
    Delete {
        #[arg(short, long)]
        id: u32,
    },
    // List
    List,

    // Summary
    Summary,

    // Exit
    Quit
}
