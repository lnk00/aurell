use aurell_types::SendMagicLinkRequest;
use dioxus::{logger::tracing::info, prelude::*};
use dioxus_query::{mutation::Mutation, prelude::use_mutation};
use query::SendMagicLinkMutation;

mod http_client;
mod query;

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
    let send_magic_link = use_mutation(Mutation::new(SendMagicLinkMutation));

    info!("{:?}", send_magic_link.read().state());

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
                                send_magic_link.mutate(SendMagicLinkRequest {
                                    email: email.read().clone(),
                                })
                            },
                            "Continue withe email"
                            if send_magic_link.read().state().is_loading() {
                                span {
                                    class:"loading loading-spinner"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
