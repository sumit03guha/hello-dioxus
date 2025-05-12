use dioxus::prelude::*;

use crate::components::Counter;

#[component]
pub fn DisplayCounter() -> Element {
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
