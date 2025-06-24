use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "h-screen w-screen flex items-center justify-center",
            div {
                "Home"
            }
        }
    }
}
