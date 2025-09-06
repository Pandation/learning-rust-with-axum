pub fn create_routes() -> Router {
    Router::new().route("/", get(root));
}


fn create_user_routes() -> Router {
    Router::new().route("/", get(root));
}