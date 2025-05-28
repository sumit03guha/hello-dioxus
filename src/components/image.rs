use dioxus::prelude::*;
pub const DIAMOND_IMAGE: Asset = asset!(
    "/assets/diamond.jpg",
    ImageAssetOptions::new()
        .with_size(ImageSize::Manual {
            width: 200,
            height: 200
        })
        .with_format(ImageFormat::Avif)
);

#[component]
pub fn ImageComponent() -> Element {
    rsx!(
        h1 { "The image is shown below." }
        img {
        src: "{DIAMOND_IMAGE}"
    })
}
