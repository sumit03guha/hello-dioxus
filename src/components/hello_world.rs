use dioxus::prelude::*;

#[component]
pub fn HelloWorld() -> Element {
    let navigator = use_navigator();
    rsx!(
        div {"Hello World"}
        button {
            onclick: move |_| {navigator.go_back()},
            "Go back"
        }
    )
}
