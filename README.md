# Expense Tracker API

A REST API for tracking expenses, built in Rust with the [Axum](https://github.com/tokio-rs/axum) web framework and the [Tokio](https://tokio.rs) async runtime. Expense data is persisted in a **PostgreSQL** database and accessed through [SQLx](https://github.com/launchbadge/sqlx), with database migrations applied automatically on startup.

This project is an implementation of the [Expense Tracker](https://roadmap.sh/projects/expense-tracker) challenge from [roadmap.sh](https://roadmap.sh). It has evolved from an earlier command-line/REPL prototype into a web API.

## Features

- Full CRUD REST endpoints for expenses (create, read, update, delete, list)
- Partial updates: `PATCH` accepts only the fields you want to change
- Category tagging via a fixed set of categories (`food`, `transport`, `housing`, `utilities`, `entertainment`)
- Per-expense timestamps: `created_at` and `updated_at` (`updated_at` is refreshed automatically on edits)
- Month tracking: each expense records the month it was created (`month` field, 1–12)
- Request validation for descriptions and amounts
- PostgreSQL persistence with schema migrations run at startup
- Layered architecture: HTTP handler → service → repository
- Request logging via a `tower-http` `TraceLayer`
- Graceful shutdown on `Ctrl+C`

## Requirements

- Rust and Cargo — this project uses the 2024 edition, so install a recent toolchain via [rustup](https://rustup.rs/) (Rust 1.85 or later)
- A running [PostgreSQL](https://www.postgresql.org/) server

## Dependencies

The project uses the following crates (see [Cargo.toml](Cargo.toml)); all are fetched automatically by Cargo when you build:

| Crate                | Purpose                                          |
|----------------------|--------------------------------------------------|
| `axum`               | Web framework, routing, and HTTP handlers        |
| `tokio`              | Asynchronous runtime (`#[tokio::main]`)          |
| `sqlx`               | Typed PostgreSQL access and migrations           |
| `serde` / `serde_json` | (De)serialization of request/response JSON     |
| `chrono`             | Timestamps (`created_at` / `updated_at`)         |
| `validator`          | Request body validation                          |
| `thiserror`          | Typed error definitions (`AppError`)             |
| `dotenvy`            | Loads configuration from a `.env` file           |
| `tracing` / `tracing-subscriber` | Structured logging                  |
| `tower-http`                     | HTTP request logging (`TraceLayer`) |

## Setup

### 1. Clone and build

```bash
git clone https://github.com/Galadima3/expense-tracka-api
cd expense-tracka-api
cargo build
```

### 2. Configure the database

Create a PostgreSQL database, then create a `.env` file in the project root:

```bash
# Required — the database connection string
DATABASE_URL=postgres://USER:PASSWORD@localhost:5432/expense_tracker

# Optional — where the server binds; defaults to 127.0.0.1:3000
SERVER_ADDR=127.0.0.1:3000
```

The `.env` file is gitignored (see [.gitignore](.gitignore)). Configuration is read from the environment in [core/config.rs](src/core/config.rs): `DATABASE_URL` is required, while `SERVER_ADDR` is optional and falls back to `127.0.0.1:3000`.

### 3. Run the server

```bash
cargo run
```

On startup the app connects to the database and automatically runs any pending migrations in the [migrations/](migrations) directory, creating the `categories` enum and `expenses` table if they don't exist yet (see [core/db.rs](src/core/db.rs)). The server then listens on the address set by `SERVER_ADDR` (default `127.0.0.1:3000`) and shuts down gracefully when you press `Ctrl+C`.

## Usage

### Endpoints

| Method | Path            | Description                                       | Success status |
|--------|-----------------|---------------------------------------------------|----------------|
| `GET`  | `/`             | Root health check (returns `Hello World!`)        | 200 |
| `GET`  | `/expense`      | List all expenses (newest first)                  | 200 |
| `POST` | `/expense`      | Create a new expense                              | 201 |
| `GET`  | `/expense/{id}` | Fetch a single expense by ID                      | 200 |
| `PATCH` | `/expense/{id}` | Update an existing expense (partial)            | 200 |
| `DELETE` | `/expense/{id}` | Delete an expense by ID                         | 204 |

The routes are defined in [main.rs](src/main.rs): the expense sub-routes are mounted under the `/expense` path.

### Example requests

Create an expense:

```bash
curl -X POST http://127.0.0.1:3000/expense \
  -H "Content-Type: application/json" \
  -d '{"description": "Lunch", "amount": 1500, "category": "food"}'
```

List all expenses:

```bash
curl http://127.0.0.1:3000/expense
```

Fetch a single expense:

```bash
curl http://127.0.0.1:3000/expense/1
```

Partially update an expense (only include the fields you want to change):

```bash
curl -X PATCH http://127.0.0.1:3000/expense/1 \
  -H "Content-Type: application/json" \
  -d '{"amount": 1800}'
```

Delete an expense:

```bash
curl -X DELETE http://127.0.0.1:3000/expense/1
```

### Request bodies

`POST /expense` ([ExpenseRequest](src/dto.rs)) — all fields required:

| Field         | Type   | Rules                                        |
|---------------|--------|----------------------------------------------|
| `description` | string | Required, cannot be empty                    |
| `amount`      | number | Required, must be greater than 0             |
| `category`    | string | Required, one of the valid category values   |

`PATCH /expense/{id}` ([UpdateExpenseRequest](src/dto.rs)) — all fields optional:

| Field         | Type   | Rules                                        |
|---------------|--------|----------------------------------------------|
| `description` | string | Optional, cannot be empty if provided        |
| `amount`      | number | Optional, must be greater than 0 if provided |
| `category`    | string | Optional, one of the valid category values   |

For `PATCH`, omitted fields are left unchanged in the database (`COALESCE`-based update in [repository.rs](src/repository.rs)).

### Categories

Valid values for `category`:

- `food`
- `transport`
- `housing`
- `utilities`
- `entertainment`

### Response shape

Expenses are returned as JSON objects shaped like this:

```json
{
  "id": 1,
  "description": "Lunch",
  "amount": 1500,
  "created_at": "2026-09-05T12:00:00.000000Z",
  "updated_at": "2026-09-05T12:00:00.000000Z",
  "month": 9,
  "category": "food"
}
```

| Field         | Type   | Description                                        |
|---------------|--------|----------------------------------------------------|
| `id`          | number | Auto-incremented unique identifier (serial)        |
| `description` | string | Short description of the expense                   |
| `amount`      | number | Cost as a whole number                             |
| `created_at`  | string | Creation timestamp (UTC, RFC 3339)                 |
| `updated_at`  | string | Last-modified timestamp (UTC, RFC 3339)            |
| `month`       | number | Month of creation (1–12), captured from UTC now    |
| `category`    | string | Category, one of the valid category values         |

### Error responses

Errors are mapped to HTTP status codes by [`AppError`](src/core/error.rs):

| Status | Meaning                                             |
|--------|-----------------------------------------------------|
| 400    | Malformed JSON or request validation failure        |
| 404    | No expense found for the given ID                   |
| 500    | Internal / database error                           |

## Data storage

Expenses live in a PostgreSQL `expenses` table. The schema is created and versioned by the migration in [migrations/20260904070334_create_expenses_table.sql](migrations/20260904070334_create_expenses_table.sql):

- `categories` enum with the five valid categories
- `expenses` table with a `SERIAL` id, `description`, `amount` (`CHECK (amount >= 0)`), `created_at`/`updated_at` (`TIMESTAMPTZ`, default `now()`), `month` (`CHECK (month BETWEEN 1 AND 12)`), and a `category` column backed by the enum

Migrations run automatically on every startup via `sqlx::migrate!()`.

## Project structure

```
expense-tracka-api/
├── Cargo.toml                        # Package manifest and dependencies
├── .env                              # DATABASE_URL (gitignored)
├── migrations/
│   └── 20260904070334_create_expenses_table.sql
└── src/
    ├── main.rs                       # Entry point, router + middleware, HTTP server
    ├── core.rs                       # Module declarations for core/*
    ├── core/
    │   ├── app_state.rs              # Shared AppState (Postgres connection pool)
    │   ├── config.rs                 # Configuration from environment (DATABASE_URL)
    │   ├── db.rs                     # Connection pool init + auto migrations
    │   └── error.rs                  # AppError and HTTP response mapping
    ├── dto.rs                        # Create/update request DTOs with validation
    ├── model.rs                      # Expense struct and Categories enum
    ├── repository.rs                 # SQL queries against PostgreSQL
    ├── service.rs                    # Business-logic layer
    └── handler.rs                    # Axum HTTP handlers
```

The request flow is `handler` → `service` → `repository`, with shared state (the database pool) threaded through via `AppState`.

## Roadmap / known limitations

- Only PostgreSQL is supported (the connection pool is a `PgPool`)
- Amounts are stored as whole integers — no decimal/fractional currency support yet
- `GET /expense` returns all rows with no pagination or filtering options
- `AppError::Conflict` is defined but not currently produced by any code path
- `expenses.json` in the project root is a leftover data file from the earlier CLI prototype; the app no longer reads or writes it
