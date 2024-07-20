use crate::error::AppError;
use crate::models::{BundleProduct, Product, ProductBundle};
use crate::repositories::ProductRepository;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait ProductService: Send + Sync {
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
}

pub struct ProductServiceImpl {
    product_repository: Arc<dyn ProductRepository>,
}

impl ProductServiceImpl {
    pub fn new(product_repository: Arc<dyn ProductRepository>) -> Self {
        Self { product_repository }
    }
}

#[async_trait]
impl ProductService for ProductServiceImpl {
    async fn get_all_products(&self) -> Result<Vec<Product>, AppError> {
        self.product_repository.get_all_products().await
    }

    async fn get_product(&self, id: i32) -> Result<Product, AppError> {
        self.product_repository.get_product(id).await
    }

    async fn create_product(&self, product: Product) -> Result<Product, AppError> {
        self.product_repository.create_product(product).await
    }

    async fn update_product(&self, product: Product) -> Result<Product, AppError> {
        self.product_repository.update_product(product).await
    }

    async fn delete_product(&self, id: i32) -> Result<(), AppError> {
        self.product_repository.delete_product(id).await
    }

    async fn get_all_bundles(&self) -> Result<Vec<ProductBundle>, AppError> {
        self.product_repository.get_all_bundles().await
    }

    async fn get_bundle(&self, id: i32) -> Result<ProductBundle, AppError> {
        self.product_repository.get_bundle(id).await
    }

    async fn create_bundle(
        &self,
        bundle: ProductBundle,
        products: Vec<BundleProduct>,
    ) -> Result<ProductBundle, AppError> {
        self.product_repository
            .create_bundle(bundle, products)
            .await
    }

    async fn update_bundle(
        &self,
        bundle: ProductBundle,
        products: Vec<BundleProduct>,
    ) -> Result<ProductBundle, AppError> {
        self.product_repository
            .update_bundle(bundle, products)
            .await
    }

    async fn delete_bundle(&self, id: i32) -> Result<(), AppError> {
        self.product_repository.delete_bundle(id).await
    }
}
