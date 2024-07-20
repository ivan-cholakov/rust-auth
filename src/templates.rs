use crate::models::{Product, ProductBundle, User};
use askama::Template;

#[derive(Template)]
#[template(path = "users.html")]
pub struct UsersTemplate {
    pub users: Vec<User>,
}

#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate {}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {}

#[derive(Template)]
#[template(path = "products/list.html")]
pub struct ProductListTemplate {
    pub products: Vec<Product>,
}

#[derive(Template)]
#[template(path = "products/detail.html")]
pub struct ProductDetailTemplate {
    pub product: Product,
}

#[derive(Template)]
#[template(path = "products/form.html")]
pub struct ProductFormTemplate {
    pub product: Option<Product>,
    pub action: String,
}

#[derive(Template)]
#[template(path = "bundles/list.html")]
pub struct BundleListTemplate {
    pub bundles: Vec<ProductBundle>,
}

#[derive(Template)]
#[template(path = "bundles/detail.html")]
pub struct BundleDetailTemplate {
    pub bundle: ProductBundle,
    pub products: Vec<(Product, i32)>, // (Product, quantity)
}

#[derive(Template)]
#[template(path = "bundles/form.html")]
pub struct BundleFormTemplate {
    pub bundle: Option<ProductBundle>,
    pub all_products: Vec<Product>,
    pub selected_products: Vec<(Product, i32)>, // (Product, quantity)
    pub action: String,
}
