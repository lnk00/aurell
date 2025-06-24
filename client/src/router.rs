use crate::pages::home::Home;
use crate::pages::signin::Signin;
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/signin")]
    Signin {},
}

#[component]
pub fn ClientRouter() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
