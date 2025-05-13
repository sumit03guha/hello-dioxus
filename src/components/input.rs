use dioxus::prelude::*;

#[component]
pub fn InputComponent() -> Element {
    let mut input_text = use_signal(|| "".to_string());

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
