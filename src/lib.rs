pub mod components;
pub mod models;
pub mod views;

use dioxus::prelude::*;
use views::{AuthCallback, Blog, Forum, Home, Members, Navbar, Profile};

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog")]
    Blog {},
    #[route("/members")]
    Members {},
    #[route("/forum")]
    Forum {},
    #[route("/profile")]
    Profile {},
    #[route("/auth/callback")]
    AuthCallback {},
}
