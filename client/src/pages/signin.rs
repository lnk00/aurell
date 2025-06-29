use aurell_types::SendMagicLinkRequest;
use dioxus::prelude::*;
use dioxus_query::{mutation::Mutation, prelude::use_mutation};

use crate::{api, router::Route};

#[component]
pub fn Signin() -> Element {
    let mut email = use_signal(|| "".to_string());
    let send_magic_link = use_mutation(Mutation::new(api::auth::SendMagicLinkMutation));
    let nav = use_navigator();

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
                            onclick: move |_| async move {
                                send_magic_link.mutate_async(SendMagicLinkRequest {
                                    email: email.read().clone(),
                                }).await;
                                nav.push(Route::MagicLinkSent {});
                            },
                            "Continue withe email"
                            if send_magic_link.read().state().is_loading() {
                                span {
                                    class:"loading loading-spinner loading-xs"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
