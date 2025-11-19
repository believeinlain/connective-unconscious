use axum_extra::response::Css;
use maud::{DOCTYPE, Markup, html};

pub async fn style() -> Css<&'static str> {
    const STYLE_CSS: &str = include_str!("../include/style.css");

    Css(STYLE_CSS)
}

pub fn header(title: &str) -> Markup {
    html! {
        div class="header" {
            h1 { (title) }
            div class="navbar" {
                a href="/" { "Home" }
                a href="/gallery" { "Gallery" }
                a href="/demos" { "Demos" }
            }
        }
    }
}

pub fn footer() -> Markup {
    html! {
        div class="footer" {
            p {
                "Website by Stephanie Aelmore. Source available "
                a href="https://github.com/believeinlain/connective-unconscious" {
                    "here "
                }
                "under the MIT License."
            }
            a class="lock" href="/login" { "🔒" }
        }
    }
}

pub fn page(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="utf-8";
            link rel="stylesheet" type="text/css" href="/style.css";
            title { (title) }
        }
        body {
            (header(title))
            div class="content" {
                (content)
            }
            (footer())
        }
    }
}
