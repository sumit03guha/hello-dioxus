use crate::components::HelloWorld;
use dioxus::prelude::*;

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[route("/hello-world")]
    HelloWorld {},
}
