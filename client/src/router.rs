use crate::pages::authenticate_magic_link::AuthenticateMagicLink;
use crate::pages::home::Home;
use crate::pages::magic_link_sent::MagicLinkSent;
use crate::pages::signin::Signin;
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/signin")]
    Signin {},
    #[route("/auth/magiclink/sent")]
    MagicLinkSent {},
    #[route("/auth/magiclink/authenticate")]
    AuthenticateMagicLink {},
}

#[component]
pub fn ClientRouter() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
