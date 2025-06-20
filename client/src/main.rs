use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div {
            class: "h-screen w-screen flex items-center justify-center",
            div {
                class: "card w-96",
                div {
                    class: "card-body",
                    div {
                        class: "card-title text-5xl font-bold mb-6",
                        "Login"
                    }
                    input {
                        class: "input input-bordered w-full",
                        placeholder: "Enter your email",
                        type: "email",
                    }
                    p {
                        class: "label opacity-50",
                        "exemple: john.doe@example.com"
                    }
                    div {
                        class: "card-actions mt-4",

                        button {
                            class: "btn btn-block",
                            type: "submit",
                            "Continue withe email"
                        }
                    }
                }
            }
        }
    }
}
