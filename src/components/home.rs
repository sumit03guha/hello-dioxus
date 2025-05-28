use dioxus::{logger::tracing, prelude::*};

use crate::routes::Route;

#[component]
pub fn Home() -> Element {
    let navigator = use_navigator();
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
        br {}
        Link {to: Route::ImageComponent { }, "Image"}
        br {  }
        Link {to: Route::ResourceComponent {  }, "Resource"}
        br {  }

        button {
            onclick: move |_| {
                match navigator.push(Route::HelloWorld { })
                {
                    None => {},
                    Some(f) => tracing::error!("Navigation error : {:#?}", f)
                }
            },
            "Go to Hello World"
        }
    )
}
