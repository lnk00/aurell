use aurell_types::{SendMagicLinkRequest, SendMagicLinkResponse};
use dioxus::{logger::tracing::info, prelude::*};

mod http_client;

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
    let mut email = use_signal(|| "".to_string());

    let fetch_new = move |_: ()| async move {
        let request_body = SendMagicLinkRequest {
            email: email.read().as_str().to_string(),
        };

        let response = http_client::post::<SendMagicLinkRequest, SendMagicLinkResponse>(
            "/auth/magiclink/send",
            &request_body,
        )
        .await;

        info!("Response: {:?}", response);
    };

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
                        autocomplete: "email",
                        value: email,
                        oninput: move |event| email.set(event.value())
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
                            onclick: move |_| {
                                spawn(fetch_new(()));
                            },
                            "Continue withe email"
                        }
                    }
                }
            }
        }
    }
}
