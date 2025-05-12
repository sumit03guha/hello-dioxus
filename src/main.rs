#![allow(non_snake_case)]

mod components;
mod routes;

use components::{
    ConditionalDiv, Counter, CounterComponent, DisplayCounter, InputComponent, Person,
    PersonComponent,
};
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
    rsx!(Router::<Route> {})
}

pub fn Home() -> Element {
    let person = Person {
        name: "Alice".to_string(),
        age: 23,
    };

    let counter: Signal<Counter> = use_signal(|| Counter { value: 0 });

    use_context_provider(|| counter);

    let input_text = use_signal(|| "".to_string());

    rsx! {
        document::Stylesheet { href: CSS }
        "Hello World!",
        div {
            class: "c",
            "Hello parent div",
            div { class: "b", "Hello inner div" }
            h1 { class: "a", "Hello inner h1" }
            PersonComponent { person }
        },
        div {
            "New div",
            button { onclick: |_| {tracing::info!("Button clicked")},  class: "button_1", "Click Me!" }
        }
        CounterComponent { }
        DisplayCounter {}
        ConditionalDiv { }

        div {
            "Input Div"
            InputComponent { input_text }
        }
    }
}
