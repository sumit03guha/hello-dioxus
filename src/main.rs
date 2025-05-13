#![allow(non_snake_case)]

mod components;
mod routes;

use components::Counter;
use dioxus::{logger::tracing, prelude::*};
use routes::Route;

static CSS: Asset = asset!("/assets/main.css");

fn main() {
    launch(|| {
        tracing::info!("Rendering app!");
        App()
    });
}

fn App() -> Element {
    let counter: Signal<Counter> = use_signal(|| Counter { value: 0 });
    use_context_provider(|| counter);

    rsx!(
        document::Stylesheet { href: CSS }
        Router::<Route> {}
    )
}
