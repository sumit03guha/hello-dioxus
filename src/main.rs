#![allow(non_snake_case)]

use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/main.css");

fn main() {
    launch(App);
}

fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        "Hello World!",
        div {
            class: "c",
            "Hello parent div",
            div { class: "b", "Hello inner div" }
            h1 { class: "a", "Hello inner h1" }
            NewComponent {}
        }
    }
}

fn NewComponent() -> Element {
    rsx!(div { background_color: "pink", color: "blue", "Hello new component" })
}
