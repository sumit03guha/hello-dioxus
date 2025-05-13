use crate::components::{
    ConditionalDiv, CounterComponent, DivExamples, HelloWorld, Home, InputComponent, Person,
    PersonComponent,
};

use dioxus::prelude::*;

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/hello-world")]
    HelloWorld {},
    #[route("/div-examples")]
    DivExamples {},
    #[route("/conditional-div")]
    ConditionalDiv {},
    #[route("/counter")]
    CounterComponent {},
    #[route("/input")]
    InputComponent {},
    #[route("/person")]
    PersonComponent { person: Person },
}
