pub mod components;
pub mod models;
pub mod views;

use dioxus::prelude::*;
use views::{AuthCallback, Home, Navbar, Profile};

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/profile")]
    Profile {},
    #[route("/auth/callback")]
    AuthCallback {},
}
