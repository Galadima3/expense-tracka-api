use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Expense {
    pub id: i32,
    pub description: String,
    pub amount: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub month: i16,
    pub category: Categories,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "categories", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Categories {
    Food,
    Transport,
    Housing,
    Utilities,
    Entertainment,
}
