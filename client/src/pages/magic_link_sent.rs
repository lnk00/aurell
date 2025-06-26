use dioxus::prelude::*;

#[component]
pub fn MagicLinkSent() -> Element {
    rsx! {
        div {
            class: "h-screen w-screen flex items-center justify-center",
            div {
                "Magic Link Sent"
            }
        }
    }
}
