#![allow(non_snake_case)]

use dioxus::{logger::tracing, prelude::*};

static CSS: Asset = asset!("/assets/main.css");

#[derive(Clone, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    launch(|| {
        tracing::info!("Rendering app!");
        App()
    });
}

struct Counter {
    value: i32,
}

fn App() -> Element {
    let person = Person {
        name: "Alice".to_string(),
        age: 23,
    };

    let counter: Signal<Counter> = use_signal(|| Counter { value: 0 });

    use_context_provider(|| counter);

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
        CounterComponent { }
        DisplayCounter {}
        ConditionalDiv {  }
    }
}

#[component]
fn NewComponent(person: Person) -> Element {
    rsx!(div { background_color: "pink", color: "blue", "Hello {person.name}, you are {person.age} years old." })
}

#[component]
fn CounterComponent() -> Element {
    let mut counter = use_context::<Signal<Counter>>();

    rsx!(
        div {
            class: "c",
            "Counter Div",
            div { "Counter : {counter.read().value}" },
                button { onclick: move |_| counter.write().value +=1 , "Increase Counter"},
                button { onclick: move |_| counter.write().value -=1 , "Decrease Counter"},
                button { onclick: move |_| counter.set(Counter{value:0}) , "Reset Counter"},
        }
    )
}

#[component]
fn DisplayCounter() -> Element {
    let counter = use_context::<Signal<Counter>>();
    rsx!(
        div {
            class: "d",
            "Shared State Counter Div",
            div {
                "The Counter is : {counter.read().value}"
            }
        }
    )
}

#[component]
fn ConditionalDiv() -> Element {
    let is_hidden = false;
    let is_blue_colour = false;

    rsx!(div { hidden: is_hidden, color: if is_blue_colour { "blue" } else { "green" }, "This Div can be hidden" })
}
