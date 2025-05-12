use dioxus::prelude::*;

#[component]
pub fn InputComponent(input_text: Signal<String>) -> Element {
    rsx!(
        div { "You have entered: {input_text}" }
        form {
            onsubmit: move |_| input_text.set("".to_string()),
            input {
                placeholder: "Enter...",
                value: input_text,
                oninput: move |e| input_text.set(e.value()),
            }
        }
    )
}
