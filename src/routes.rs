use crate::components::{
    ConditionalDiv, CounterComponent, HelloWorld, InputComponent, Person, PersonComponent,
};
use crate::Home;
use dioxus::prelude::*;

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[route("/hello-world")]
    HelloWorld {},
    #[route("/")]
    Home {},
    #[route("/conditional-div")]
    ConditionalDiv {},
    #[route("/counter")]
    CounterComponent {},
    #[route("/input")]
    InputComponent { input_text: Signal<String> },
    #[route("/person")]
    PersonComponent { person: Person },
}
