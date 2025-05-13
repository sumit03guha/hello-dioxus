use dioxus::prelude::*;

#[derive(Default)]
pub struct Counter {
    pub value: i32,
}

#[component]
pub fn CounterComponent() -> Element {
    rsx!(
        div {
            DisplayCounter {  }
            CounterOps {  }
        }
    )
}

#[component]
fn CounterOps() -> Element {
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
