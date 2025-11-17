use std::{net::SocketAddrV4, str::FromStr};

use axum::{
    Json, Router, http::StatusCode, response::Html, routing::{get, get_service, post}
};
use axum_extra::response::Css;
use hypertext::prelude::*;
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let listen_address = SocketAddrV4::from_str("0.0.0.0:3000")
        .inspect_err(|err| log::error!("{err}"))
        .unwrap();

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/style.css", get(style))
        .route("/users", post(create_user))
        .nest_service("/images", get_service(ServeDir::new("static/images")));

    // run our app with hyper, listening globally on port 3000
    log::info!("Listening on {listen_address}");
    let listener = tokio::net::TcpListener::bind(listen_address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> Html<String> {
    let shopping_list = ["milk", "eggs", "bread"];
    let shopping_list_rsx = rsx! {
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="utf-8">
            <link rel="stylesheet" href="style.css">
            <title>Hello</title>
        </head>
        <body>
        <div class="content">
            <h1>Shopping List</h1>
            <ul>
                @for (i, item) in (1..).zip(shopping_list) {
                    <li class="item">
                        <input id={ "item-" (i) } type="checkbox">
                        <label for={ "item-" (i) }>(item)</label>
                    </li>
                }
            </ul>
        </div>
        </body>
        </html>
    }
    .render();

    Html::from(shopping_list_rsx.into_inner())
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

async fn style() -> Css<&'static str> {
    const STYLE_CSS: &str = include_str!("../../include/style.css");

    Css(STYLE_CSS)
}
