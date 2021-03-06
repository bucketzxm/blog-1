use tera::Context;
use dotenv::dotenv;
use std::env;

use util::auth::User;
use dal::models::visitor_log::*;
use dal::models::user;
use dal::models::post::Post;
use dal::diesel_pool::DB;

#[derive(Serialize)]
pub enum ResponseEnum {
    SUCCESS,
    FAILURE,
    ERROR,
}

#[derive(Serialize)]
pub enum ContextEnum<'a, T> {
    String(String),
    Vec(Vec<T>),
    Post(Option<&'a Post>),
    User(Option<&'a user::User>),
}
pub fn template_context(db: &DB, user: User) -> Context {
    let visitor_logs = VisitorLog::query_login_user(db.conn(), user.0);
    let users = user::User::query_by_id(db.conn(), user.0);
    // let mut context = HashMap::new();
    let mut context = Context::new();
    if let Some(user) = users.first() {
        context.add("username", &user.username);
    }
    if let Some(log) = visitor_logs.first() {
        context.add("access_time", &log.access_time);
    }
    context
}

pub fn footer_context() -> Context {
    dotenv().ok();
    let email_url = env::var("EMAIL_URL").expect("EMAIL_URL must be set");
    let stackoverflow_url = env::var("STACKOVERFLOW_URL").expect("STACKOVERFLOW_URL must be set");
    let github_url = env::var("GITHUB_URL").expect("GITHUB_URL must be set");
    let mut context = Context::new();
    context.add("email", &email_url);
    context.add("stackoverflow", &stackoverflow_url);
    context.add("github", &github_url);
    context
}
