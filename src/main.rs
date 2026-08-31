use clap::{CommandFactory, Parser};
use std::io::{self, Write};


mod model;
mod repository;

use crate::{
    model::{Commands, ExpenseRequest, UpdateExpenseRequest, Cli},
    repository::{
        create_expense, delete_expense, get_specific_expense, get_summary_of_expense,
        list_expenses, update_expense,
    },
};

const FILE_NAME: &str = "expenses.json";

fn main() {
    println!("Expense Tracker\n");
    // Special use of unwrap() here
    Cli::command().print_help().unwrap();
    println!("\n\nType `help` to see this again, or `quit` to exit.\n");

    loop {
        print!("\n> ");
        if io::stdout().flush().is_err() {
            eprintln!("Failed to flush stdout.");
        }

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read input, try again.");
            continue;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        if input == "help" {
            if let Err(err) = Cli::command().print_help() {
                eprintln!("Failed to print help: {err}")
            }
            println!();
            continue;
        }

        let tokenized_args = std::iter::once("expense-tracker").chain(input.split_whitespace());

        let args = match Cli::try_parse_from(tokenized_args) {
            Ok(args) => args,
            Err(error) => {
                println!("{error}");
                continue;
            }
        };

        match args.command {
            Commands::Quit => {
                println!("Goodbye.");
                break;
            }

            Commands::Add {
                description,
                amount,
                category,
            } => {
                let request = ExpenseRequest {
                    description,
                    amount,
                    category,
                };
                if let Err(error) = create_expense(FILE_NAME, request) {
                    eprintln!("Error creating expense: {error}");
                }
            }

            Commands::Fetch { id } => match get_specific_expense(id, FILE_NAME) {
                Ok(Some(expense)) => {
                    println!("{:#?}", expense);
                }
                Ok(None) => {
                    println!("Expense with ID {id} not found");
                }
                Err(error) => {
                    eprintln!("Error fetching expense: {error}");
                }
            },

            // Update Expense
            Commands::Update {
                id,
                description,
                amount,
                category,
            } => {
                let request = UpdateExpenseRequest {
                    description,
                    amount,
                    category,
                };

                match update_expense(id, FILE_NAME, request) {
                    Ok(()) => {
                        println!("Expense {id} updated successfully.");
                    }
                    Err(error) => {
                        eprintln!("Error updating expense: {error}");
                    }
                }
            }

            // Delete Expense
            Commands::Delete { id } => match delete_expense(id, FILE_NAME) {
                Ok(()) => {
                    println!("Expense {id} deleted successfully.");
                }
                Err(error) => {
                    eprintln!("Error deleting expense: {error}");
                }
            },

            // Expense Summary
            Commands::Summary => match get_summary_of_expense(FILE_NAME) {
                Ok(total) => {
                    println!("Total expenses: {total}");
                }
                Err(error) => {
                    eprintln!("Error calculating summary: {error}");
                }
            },

            Commands::List => match list_expenses(FILE_NAME) {
                Ok(()) => {}
                Err(error) => {
                    eprintln!("Error listing expenses: {}", error);
                }
            },
        }
    }
}
