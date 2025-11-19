use std::{collections::HashMap, net::SocketAddrV4, str::FromStr};

use axum::{
    Json, Router,
    extract::Query,
    http::StatusCode,
    routing::{get, get_service, post},
};
use maud::{DOCTYPE, Markup, html};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;

use connective_unconscious::{gallery::NIHONNOYUME, page, style};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let listen_address = SocketAddrV4::from_str("0.0.0.0:3000")
        .inspect_err(|err| log::error!("{err}"))
        .unwrap();

    // build our application with a route
    let app = Router::new()
        .route("/", get(home))
        .route("/gallery", get(gallery))
        .route("/gallery/nihonnoyume", get(nihonnoyume))
        .route("/style.css", get(style))
        .route("/users", post(create_user))
        .nest_service("/images", get_service(ServeDir::new("/static/images")));

    // run our app with hyper, listening globally on port 3000
    log::info!("Listening on {listen_address}");
    let listener = tokio::net::TcpListener::bind(listen_address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn nihonnoyume(Query(params): Query<HashMap<String, i32>>) -> Markup {
    let idx: usize = params
        .get("image")
        .copied()
        .unwrap_or_default()
        .try_into()
        .unwrap_or_default();

    let (image, caption) = NIHONNOYUME[idx];

    page(
        "日本の夢 : Connective Unconscious",
        html! {
            h2 { "Welcome to the Connective Unconscious" }
            p { (image) }
            p { (caption) }
        },
    )
}

async fn home() -> Markup {
    page(
        "Connective Unconscious",
        html! {
            h2 { "Welcome to the Connective Unconscious" }
            p { "Feel free to check out the different pages I have here via the navbar." }
            p { "More updates coming soon." }
            p { "Thank you for visiting!" }
        },
    )
}

async fn gallery() -> Markup {
    page(
        "Gallery : Connective Unconscious",
        html! {
            h2 { "Welcome to the Gallery" }
            p {
                r#"A collection of photos in the public domain that I've taken,
                assembled into meaningful sequences."#
            }
            ul {
                li {
                    a href="/gallery/nihonnoyume/" { "日本の夢" }
                    " An illustrated poem, half-remembered from disjointed dreams."
                }
                li {
                    a href="/gallery/jikannoyume/" { "時間の夢" }
                    " The passage of time."
                }
            }
        },
    )
}

async fn _root() -> Markup {
    let shopping_list = ["milk", "eggs", "bread"];
    let title = "Connective Unconscious";
    let content = html! {
        div class="content" {
            h1 { "Shopping List" }
            ul {
                @for (i, item) in (1..).zip(shopping_list) {
                    li.item {
                        input #{ "item-" (i) } type="checkbox";
                        label for={ "item-" (i) } { (item) }
                    }
                }
            }
        }
    };
    html! {
        (DOCTYPE)
        head {
            meta charset="utf-8";
            link rel="stylesheet" type="text/css" href="/style.css";
            title { "Hello" }
        }
        body {
            (page(title, content))
        }
    }
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
