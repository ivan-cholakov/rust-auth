use crate::error::AppError;
use crate::models::{BundleProduct, Product, ProductBundle};
use crate::routes::api_v1::AppState;
use crate::templates::{
    BundleDetailTemplate, BundleFormTemplate, BundleListTemplate, ProductDetailTemplate,
    ProductFormTemplate, ProductListTemplate,
};
use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::{Path, State},
    response::Html,
    Json,
};

#[derive(Template)]
#[template(path = "products.html")]
struct ProductsTemplate {
    products: Vec<Product>,
}

#[derive(Template)]
#[template(path = "product_bundles.html")]
struct ProductBundlesTemplate {
    bundles: Vec<ProductBundle>,
}

pub async fn get_products(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let products = state.product_service.get_all_products().await?;
    let template = ProductListTemplate { products };
    Ok(Html(template.render().unwrap()))
}

pub async fn get_product(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let product = state.product_service.get_product(id).await?;
    let template = ProductDetailTemplate { product };
    Ok(Html(template.render().unwrap()))
}

pub async fn create_product(
    State(state): State<AppState>,
    Json(product): Json<Product>,
) -> Result<Json<Product>, AppError> {
    let created_product = state.product_service.create_product(product).await?;
    Ok(Json(created_product))
}

pub async fn update_product(
    State(state): State<AppState>,
    Json(product): Json<Product>,
) -> Result<Json<Product>, AppError> {
    let updated_product = state.product_service.update_product(product).await?;
    Ok(Json(updated_product))
}

pub async fn delete_product(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<(), AppError> {
    state.product_service.delete_product(id).await?;
    Ok(())
}

pub async fn get_bundles(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let bundles = state.product_service.get_all_bundles().await?;
    let template = ProductBundlesTemplate { bundles };
    Ok(Html(template.render().unwrap()))
}

pub async fn get_bundle(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ProductBundle>, AppError> {
    let bundle = state.product_service.get_bundle(id).await?;
    Ok(Json(bundle))
}

pub async fn create_bundle(
    State(state): State<AppState>,
    Json((bundle, products)): Json<(ProductBundle, Vec<BundleProduct>)>,
) -> Result<Json<ProductBundle>, AppError> {
    let created_bundle = state
        .product_service
        .create_bundle(bundle, products)
        .await?;
    Ok(Json(created_bundle))
}

pub async fn update_bundle(
    State(state): State<AppState>,
    Json((bundle, products)): Json<(ProductBundle, Vec<BundleProduct>)>,
) -> Result<Json<ProductBundle>, AppError> {
    let updated_bundle = state
        .product_service
        .update_bundle(bundle, products)
        .await?;
    Ok(Json(updated_bundle))
}

pub async fn delete_bundle(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<(), AppError> {
    state.product_service.delete_bundle(id).await?;
    Ok(())
}

async fn product_detail(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let product = state.product_service.get_product(id).await?;
    let template = ProductDetailTemplate { product };
    Ok(template.into_response())
}

async fn list_products(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let products = state.product_service.get_all_products().await?;
    let template = ProductListTemplate { products };
    Ok(template.into_response())
}
