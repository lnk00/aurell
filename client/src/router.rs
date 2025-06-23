use crate::pages::home::Home;
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},
}

#[component]
pub fn ClientRouter() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
