use dioxus::prelude::*;

#[component]
pub fn AuthenticateMagicLink(token: String) -> Element {
    rsx! {
        div {
            class: "h-screen w-screen flex items-center justify-center",
            div {
                "Authenticate Magic Link"
                p { "Token: {token}" }
            }
        }
    }
}
