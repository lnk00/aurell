use dioxus::prelude::*;

#[component]
pub fn AuthenticateMagicLink() -> Element {
    rsx! {
        div {
            class: "h-screen w-screen flex items-center justify-center",
            div {
                "Authenticate Magic Link"
            }
        }
    }
}
