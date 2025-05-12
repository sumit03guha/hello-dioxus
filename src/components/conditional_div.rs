use dioxus::prelude::*;

#[component]
pub fn ConditionalDiv() -> Element {
    let is_hidden = false;
    let is_blue_colour = false;

    rsx!(div { hidden: is_hidden, color: if is_blue_colour { "blue" } else { "green" }, "This Div can be hidden" })
}
