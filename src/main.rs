#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    launch(App);
}

fn App() -> Element {
    rsx! {
        "Hello World!",
        div {
            background_color: "green",
            color: "grey",
            "Hello parent div",
            div { color: "blue", "Hello inner div" }
            h1 { color: "yellow", "Hello inner h1" }
        }
    }
}
