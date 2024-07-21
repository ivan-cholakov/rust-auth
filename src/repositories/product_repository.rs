use crate::error::AppError;
use crate::models::{BundleProduct, Product, ProductBundle};
use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Arc;
use sqlx::types::BigDecimal;

#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn get_all_products(&self) -> Result<Vec<Product>, AppError>;

    async fn get_product(&self, id: i32) -> Result<Product, AppError>;

    async fn create_product(&self, product: Product) -> Result<Product, AppError>;

    async fn update_product(&self, product: Product) -> Result<Product, AppError>;

    async fn delete_product(&self, id: i32) -> Result<(), AppError>;

    async fn get_all_bundles(&self) -> Result<Vec<ProductBundle>, AppError>;

    async fn get_bundle(&self, id: i32) -> Result<ProductBundle, AppError>;

    async fn create_bundle(
        &self,
        bundle: ProductBundle,
        products: Vec<BundleProduct>,
    ) -> Result<ProductBundle, AppError>;

    async fn update_bundle(
        &self,
        bundle: ProductBundle,
        products: Vec<BundleProduct>,
    ) -> Result<ProductBundle, AppError>;

    async fn delete_bundle(&self, id: i32) -> Result<(), AppError>;

    async fn get_bundle_products(&self, bundle_id: i32) -> Result<Vec<(Product, i32)>, AppError>;
}

pub struct ProductRepositoryImpl {
    pool: Arc<PgPool>,
}

impl ProductRepositoryImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProductRepository for ProductRepositoryImpl {
    async fn get_all_products(&self) -> Result<Vec<Product>, AppError> {
        let products = sqlx::query_as!(
            Product,
            r#"SELECT id, name, description, price as "price: BigDecimal" FROM products"#
        )
        .fetch_all(&*self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        Ok(products)
    }


    async fn get_product(&self, id: i32) -> Result<Product, AppError> {
        let product = sqlx::query_as!(
            Product,
            r#"SELECT id, name, description, price as "price: BigDecimal" FROM products WHERE id = $1"#,
            id
        )
        .fetch_one(&*self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        Ok(product)
    }

    async fn create_product(&self, product: Product) -> Result<Product, AppError> {
        let created_product = sqlx::query_as!(
            Product,
            r#"INSERT INTO products (name, description, price) 
            VALUES ($1, $2, $3) 
            RETURNING id, name, description, price as "price: BigDecimal""#,
            product.name,
            product.description,
            product.price
        )
        .fetch_one(&*self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        Ok(created_product)
    }

    async fn update_product(&self, product: Product) -> Result<Product, AppError> {
        let updated_product = sqlx::query_as!(
            Product,
            r#"UPDATE products 
            SET name = $1, description = $2, price = $3 
            WHERE id = $4 
            RETURNING id, name, description, price as "price: BigDecimal""#,
            product.name,
            product.description,
            product.price,
            product.id
        )
        .fetch_one(&*self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        Ok(updated_product)
    }

    async fn delete_product(&self, id: i32) -> Result<(), AppError> {
        sqlx::query!("DELETE FROM products WHERE id = $1", id)
            .execute(&*self.pool)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok(())
    }

    async fn get_all_bundles(&self) -> Result<Vec<ProductBundle>, AppError> {
        let bundles = sqlx::query_as!(
            ProductBundle,
            r#"SELECT id, name, description, discount_percentage as "discount_percentage: BigDecimal" FROM product_bundles"#
        )
        .fetch_all(&*self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        Ok(bundles)
    }

    async fn get_bundle(&self, id: i32) -> Result<ProductBundle, AppError> {
        let bundle = sqlx::query_as!(
            ProductBundle,
            r#"SELECT id, name, description, discount_percentage as "discount_percentage: BigDecimal" FROM product_bundles WHERE id = $1"#,
            id
        )
        .fetch_one(&*self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        Ok(bundle)
    }

    async fn create_bundle(&self, bundle: ProductBundle, products: Vec<BundleProduct>) -> Result<ProductBundle, AppError> {
        let mut tx = self.pool.begin().await.map_err(AppError::DatabaseError)?;

        let created_bundle = sqlx::query_as!(
            ProductBundle,
            r#"INSERT INTO product_bundles (name, description, discount_percentage) 
            VALUES ($1, $2, $3) 
            RETURNING id, name, description, discount_percentage as "discount_percentage: BigDecimal""#,
            bundle.name,
            bundle.description,
            bundle.discount_percentage
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(AppError::DatabaseError)?;

        for product in products {
            sqlx::query!(
                "INSERT INTO bundle_products (bundle_id, product_id, quantity) VALUES ($1, $2, $3)",
                created_bundle.id,
                product.product_id,
                product.quantity
            )
            .execute(&mut *tx)
            .await
            .map_err(AppError::DatabaseError)?;
        }

        tx.commit().await.map_err(AppError::DatabaseError)?;

        Ok(created_bundle)
    }

    async fn update_bundle(&self, bundle: ProductBundle, products: Vec<BundleProduct>) -> Result<ProductBundle, AppError> {
        let mut tx = self.pool.begin().await.map_err(AppError::DatabaseError)?;

        let updated_bundle = sqlx::query_as!(
            ProductBundle,
            r#"UPDATE product_bundles 
            SET name = $1, description = $2, discount_percentage = $3 
            WHERE id = $4 
            RETURNING id, name, description, discount_percentage as "discount_percentage: BigDecimal""#,
            bundle.name,
            bundle.description,
            bundle.discount_percentage,
            bundle.id
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(AppError::DatabaseError)?;

        sqlx::query!("DELETE FROM bundle_products WHERE bundle_id = $1", bundle.id)
            .execute(&mut *tx)
            .await
            .map_err(AppError::DatabaseError)?;

        for product in products {
            sqlx::query!(
                "INSERT INTO bundle_products (bundle_id, product_id, quantity) VALUES ($1, $2, $3)",
                updated_bundle.id,
                product.product_id,
                product.quantity
            )
            .execute(&mut *tx)
            .await
            .map_err(AppError::DatabaseError)?;
        }

        tx.commit().await.map_err(AppError::DatabaseError)?;

        Ok(updated_bundle)
    }

    async fn delete_bundle(&self, id: i32) -> Result<(), AppError> {
        let mut tx = self.pool.begin().await.map_err(AppError::DatabaseError)?;

        sqlx::query!("DELETE FROM bundle_products WHERE bundle_id = $1", id)
            .execute(&mut *tx)
            .await
            .map_err(AppError::DatabaseError)?;

        sqlx::query!("DELETE FROM product_bundles WHERE id = $1", id)
            .execute(&mut *tx)
            .await
            .map_err(AppError::DatabaseError)?;

        tx.commit().await.map_err(AppError::DatabaseError)?;

        Ok(())
    }

    async fn get_bundle_products(&self, bundle_id: i32) -> Result<Vec<(Product, i32)>, AppError> {
        let bundle_products = sqlx::query!(
            r#"
            SELECT p.id, p.name, p.description, p.price as "price: BigDecimal", bp.quantity
            FROM products p
            JOIN bundle_products bp ON p.id = bp.product_id
            WHERE bp.bundle_id = $1
            "#,
            bundle_id
        )
        .fetch_all(&*self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        let result = bundle_products
            .into_iter()
            .map(|row| {
                (
                    Product {
                        id: row.id,
                        name: row.name,
                        description: row.description,
                        price: row.price,
                    },
                    row.quantity,
                )
            })
            .collect();

        Ok(result)
    }
}
