use crate::components::HelloWorld;
use crate::Home;
use dioxus::prelude::*;

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[route("/hello-world")]
    HelloWorld {},
    #[route("/")]
    Home {},
}
