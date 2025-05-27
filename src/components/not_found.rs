use dioxus::prelude::*;

#[component]
pub fn NotFound(path: Vec<String>) -> Element {
    rsx!(
        h1 {"Not Found"}
        for p in path {
            p {"The page : {p} does not exist."}
        }
    )
}
