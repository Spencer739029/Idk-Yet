use axum::{
    response::Html,
    extract::Form,
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;
use std::{collections::HashMap, net::SocketAddr};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .route("/button", post(button_handler))
        .nest_service("/static", ServeDir::new("static"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("bind failed");

    axum::serve(listener, app)
        .await
        .expect("server failed");
}

async fn handler() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}

async fn button_handler(Form(form): Form<HashMap<String, String>>) -> String {
    if let Some(_) = form.get("button1") {
        "Me too!".to_string()
    } else {
        "What are you then...".to_string()
    }
}
