use dioxus::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct ApiResponse {
    message: String,
    status: String,
}

#[component]
pub fn ResourceComponent() -> Element {
    let resource = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<ApiResponse>()
            .await
    });

    match &*resource.read_unchecked() {
        Some(Ok(res)) => rsx!(
            h1 {"The picture : "}
            img {src: res.message.clone()}
        ),
        Some(Err(_e)) => rsx!(
            h1 { "Failed to load image" }
        ),
        None => rsx!(
            h1 { "Loading..." }
        ),
    }
}
