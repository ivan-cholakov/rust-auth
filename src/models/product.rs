use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ProductBundle {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub discount_percentage: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct BundleProduct {
    pub bundle_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}
