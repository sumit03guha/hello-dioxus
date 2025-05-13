use dioxus::prelude::*;

use crate::routes::Route;

#[component]
pub fn Home() -> Element {
    rsx!(
        div {
            "Welcome to my Dioxus Learning -> Hello Dioxus"
        }

        Link {to: Route::Home {  }, "Home"}
        br {}
        Link {to: Route::HelloWorld {  }, "Hello World"}
        br {}
        Link {to: Route::DivExamples {  }, "Div Examples"}
        br {}
        Link {to: Route::ConditionalDiv {  }, "Conditional Div"}
        br {}
        Link {to: Route::CounterComponent {  }, "Counter"}
        br {}
        Link {to: Route::InputComponent { }, "Input"}
    )
}
