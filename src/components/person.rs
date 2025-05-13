use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

impl Default for Person {
    fn default() -> Self {
        Self {
            name: "Alice".to_string(),
            age: 21,
        }
    }
}

#[component]
pub fn PersonComponent(person: Person) -> Element {
    rsx!(div { background_color: "pink", color: "blue", "Hello {person.name}, you are {person.age} years old." })
}
