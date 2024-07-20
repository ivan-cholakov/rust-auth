use crate::models::User;
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
