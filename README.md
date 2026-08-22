# Expense Tracker

A simple command-line expense tracker built in Rust. Expenses are stored locally in a JSON file, and the app runs as an interactive REPL, so you can keep issuing commands without restarting the program.

This project is an implementation of the [Expense Tracker](https://roadmap.sh/projects/expense-tracker) challenge from [roadmap.sh](https://roadmap.sh).

## Features

- Add, fetch, update, delete, and list expenses
- Partial updates: only pass the fields you want to change
- Category tagging via a fixed set of categories
- Running total summary of all expenses
- Persistent storage to a local JSON file (`expenses.json`)
- Interactive loop with built-in help, so invalid input doesn't crash the app

## Requirements

- Rust and Cargo (install via [rustup](https://rustup.rs/))

## Installation

```bash
git clone [<repo-url>](https://github.com/Galadima3/expense-tracka-cli)
cd expense-tracka-cli
cargo build --release
```

## Usage

Run the app with:

```bash
cargo run
```

On startup, the full command reference is printed. You'll then see a prompt:

```
> 
```

Type commands at the prompt one at a time. Type `help` to reprint the command reference, or `quit` to exit.

### Commands

| Command  | Description                          | Flags |
|----------|---------------------------------------|-------|
| `add`    | Add a new expense                     | `-d, --description <TEXT>` `-a, --amount <NUMBER>` `-c, --category <CATEGORY>` |
| `fetch`  | Fetch a single expense by ID          | `-i, --id <ID>` |
| `update` | Update an existing expense (partial)  | `-i, --id <ID>` `-d, --description <TEXT>` `-a, --amount <NUMBER>` `-c, --category <CATEGORY>` |
| `delete` | Delete an expense by ID               | `-i, --id <ID>` |
| `list`   | List all expenses                     | none |
| `summary`| Show the total of all expenses        | none |
| `quit`   | Exit the app                          | none |

For `update`, only include the flags for fields you want to change; omitted fields are left as-is.

### Categories

Valid values for `--category`:

- `food`
- `transport`
- `housing`
- `utilities`
- `entertainment`

### Examples

```
> add -d "Lunch" -a 1500 -c food
> add -d "Bus fare" -a 300 -c transport
> list
#1 | Lunch | 1500 | Food | 2026-08-22 10:15:03 UTC
#2 | Bus fare | 300 | Transport | 2026-08-22 10:15:20 UTC

> fetch -i 1
Expense {
    id: 1,
    description: "Lunch",
    amount: 1500,
    ...
}

> update -i 1 -a 1800
Expense 1 updated successfully.

> summary
Total expenses: 2100

> delete -i 2
Expense 2 deleted successfully.

> quit
Goodbye.
```

## Data storage

Expenses are stored in `expenses.json` in the directory the app is run from. The file is created automatically on the first successful `add`; if it doesn't exist yet, `fetch`, `list`, and `summary` simply report no data rather than erroring.

## Project structure

```
src/
├── main.rs        # REPL loop and command dispatch
├── dto.rs         # CLI argument definitions and request structs
├── model.rs       # Expense and Categories data types
└── repository.rs  # File-backed persistence and business logic
```

## Roadmap / known limitations

- Amounts are stored as whole `u32` values (no decimal/fractional currency support yet)
- Command input is split on whitespace, so quoted multi-word arguments (e.g. `-d "coffee with friends"`) aren't supported
- Error handling is functional but not yet exhaustive (see `// TODO: Add Error Handling` in `main.rs`)
