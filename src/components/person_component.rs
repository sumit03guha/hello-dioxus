use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

#[component]
pub fn PersonComponent(person: Person) -> Element {
    rsx!(div { background_color: "pink", color: "blue", "Hello {person.name}, you are {person.age} years old." })
}
