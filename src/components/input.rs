use dioxus::prelude::*;

#[component]
pub fn InputComponent() -> Element {
    let mut input_text = use_signal(|| "".to_string());
    let handle_input = use_callback(move |e: Event<FormData>| input_text.set(e.value()));

    rsx!(
        div { "You have entered: {input_text}" }
        form {
            onsubmit: move |_| input_text.set("".to_string()),
            input {
                placeholder: "Enter...",
                value: input_text,
                oninput: handle_input,
            }
        }
    )
}
