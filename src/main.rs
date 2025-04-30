#![allow(non_snake_case)]

use dioxus::{logger::tracing, prelude::*};

static CSS: Asset = asset!("/assets/main.css");

fn main() {
    launch(|| {
        tracing::info!("Rendering app!");
        App()
    });
}

fn App() -> Element {
    let person = Person {
        name: "Alice".to_string(),
        age: 23,
    };

    let counter = use_signal(|| 0);

    rsx! {
        document::Stylesheet { href: CSS }
        "Hello World!",
        div {
            class: "c",
            "Hello parent div",
            div { class: "b", "Hello inner div" }
            h1 { class: "a", "Hello inner h1" }
            NewComponent { person }
        },
        div {
            "New div",
            button { onclick: |_| {tracing::info!("Button clicked")},  class: "button_1", "Click Me!" }
        }
        CounterComponent { counter }
    }
}

#[component]
fn NewComponent(person: Person) -> Element {
    rsx!(div { background_color: "pink", color: "blue", "Hello {person.name}, you are {person.age} years old." })
}

#[component]
fn CounterComponent(counter: Signal<i32>) -> Element {
    rsx!(
        div {
            class: "c",
            "Counter Div",
            div { "Counter : {counter}" },
                button { onclick: move |_| counter +=1 , "Increase Counter"},
                button { onclick: move |_| counter -=1 , "Decrease Counter"},
                button { onclick: move |_| counter.set(0) , "Reset Counter"},
        }
    )
}

#[derive(Clone, PartialEq)]
struct Person {
    name: String,
    age: u32,
}
