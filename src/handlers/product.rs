use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use crate::error::AppError;
use crate::models::{BundleProduct, Product, ProductBundle};
use crate::routes::api_v1::AppState;
use crate::templates::{
    BundleDetailTemplate, BundleFormTemplate, BundleListTemplate, ProductDetailTemplate,
    ProductFormTemplate, ProductListTemplate,
};
use askama::Template;
use askama_axum::IntoResponse;
use axum::Form;
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
    Ok(template)
}

pub async fn get_product(State(state): State<AppState>, Path(id): Path<i32>) -> Result<impl IntoResponse, AppError> {
    let product = state.product_service.get_product(id).await?;
    let template = ProductDetailTemplate { product };
    Ok(template)
}

pub async fn new_product() -> Result<impl IntoResponse, AppError> {
    let template = ProductFormTemplate {
        product: None,
        action: "post".to_string(),
    };
    Ok(template)
}

pub async fn edit_product(State(state): State<AppState>, Path(id): Path<i32>) -> Result<impl IntoResponse, AppError> {
    let product = state.product_service.get_product(id).await?;
    let template = ProductFormTemplate {
        product: Some(product),
        action: "put".to_string(),
    };
    Ok(template)
}

pub async fn create_product(
    State(state): State<AppState>,
    Form(product): Form<Product>,
) -> Result<impl IntoResponse, AppError> {
    let created_product = state.product_service.create_product(product).await?;
    let template = ProductDetailTemplate { product: created_product };
    Ok(template)
}

pub async fn update_product(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Form(mut product): Form<Product>,
) -> Result<impl IntoResponse, AppError> {
    product.id = id;
    let updated_product = state.product_service.update_product(product).await?;
    let template = ProductDetailTemplate { product: updated_product };
    Ok(template)
}

pub async fn delete_product(State(state): State<AppState>, Path(id): Path<i32>) -> Result<impl IntoResponse, AppError> {
    state.product_service.delete_product(id).await?;
    Ok("") // Return an empty response as the product card will be removed by HTMX
}

pub struct BundleForm {
    bundle: ProductBundle,
    product_ids: Vec<i32>,
    quantities: Vec<i32>,
}

pub async fn get_bundles(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let bundles = state.product_service.get_all_bundles().await?;
    let template = BundleListTemplate { bundles };
    Ok(template)
}

pub async fn get_bundle(State(state): State<AppState>, Path(id): Path<i32>) -> Result<impl IntoResponse, AppError> {
    let bundle = state.product_service.get_bundle(id).await?;
    let products = state.product_service.get_bundle_products(id).await?;
    let template = BundleDetailTemplate { bundle, products };
    Ok(template)
}

pub async fn new_bundle(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let all_products = state.product_service.get_all_products().await?;
    let template = BundleFormTemplate {
        bundle: None,
        all_products,
        selected_products: HashMap::new(),
        action: "post".to_string(),
    };
    Ok(template)
}

pub async fn edit_bundle(State(state): State<AppState>, Path(id): Path<i32>) -> Result<impl IntoResponse, AppError> {
    let bundle = state.product_service.get_bundle(id).await?;
    let all_products = state.product_service.get_all_products().await?;
    let bundle_products = state.product_service.get_bundle_products(id).await?;
    let selected_products: HashMap<i32, i32> = bundle_products.into_iter().map(|(p, q)| (p.id, q)).collect();
    
    let template = BundleFormTemplate {
        bundle: Some(bundle),
        all_products,
        selected_products,
        action: "put".to_string(),
    };
    Ok(template)
}

pub async fn create_bundle(
    State(state): State<AppState>,
    Form(form): Form<BundleForm>,
) -> Result<impl IntoResponse, AppError> {
    let bundle_products: Vec<BundleProduct> = form.product_ids.into_iter()
        .zip(form.quantities.into_iter())
        .map(|(product_id, quantity)| BundleProduct { product_id, quantity, bundle_id: form.bundle.id })
        .collect();
    let created_bundle = state.product_service.create_bundle(form.bundle, bundle_products).await?;
    let bundle_products = state.product_service.get_bundle_products(created_bundle.id).await?;
    let template = BundleDetailTemplate { bundle: created_bundle, products: bundle_products };
    Ok(template)
}

pub async fn update_bundle(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Form(form): Form<BundleForm>,
) -> Result<impl IntoResponse, AppError> {
    let mut bundle = form.bundle;
    bundle.id = id;
    let bundle_products: Vec<BundleProduct> = form.product_ids.into_iter()
        .zip(form.quantities.into_iter())
        .map(|(product_id, quantity)| BundleProduct { product_id, quantity, bundle_id:bundle.id })
        .collect();
    let updated_bundle = state.product_service.update_bundle(id, bundle, bundle_products).await?;
    let bundle_products = state.product_service.get_bundle_products(updated_bundle.id).await?;
    let template = BundleDetailTemplate { bundle: updated_bundle, products: bundle_products };
    Ok(template)
}

pub async fn delete_bundle(State(state): State<AppState>, Path(id): Path<i32>) -> Result<impl IntoResponse, AppError> {
    state.product_service.delete_bundle(id).await?;
    Ok("") // Return an empty response as the bundle card will be removed by HTMX
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
