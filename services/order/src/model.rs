use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct OrderRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub status: String,
    pub total_amount: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct OrderItemRow {
    pub product_id: Uuid,
    pub product_name: String,
    pub unit_price: Decimal,
    pub quantity: i32,
}

/// The row shape locked via `SELECT ... FOR UPDATE` during checkout.
#[derive(Debug, FromRow)]
pub struct LockedProductRow {
    pub id: Uuid,
    pub name: String,
    pub price: Decimal,
    pub stock_quantity: i32,
}
