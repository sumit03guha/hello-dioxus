use crate::components::{
    ConditionalDiv, CounterComponent, DivExamples, HelloWorld, Home, ImageComponent,
    InputComponent, NotFound, Person, PersonComponent, ResourceComponent,
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
    #[route("/image")]
    ImageComponent {},
    #[route("/resource")]
    ResourceComponent {},
    #[route("/:..path")]
    NotFound { path: Vec<String> },
}
