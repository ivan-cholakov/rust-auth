use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::BigDecimal;

#[derive(Clone, Debug, FromRow)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: BigDecimal,
}

#[derive(Clone, Debug, FromRow)]
pub struct ProductBundle {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub discount_percentage: BigDecimal,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct BundleProduct {
    pub bundle_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}
