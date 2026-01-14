use crate::Route;
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
///
///
/// This layout component wraps the UI of [Route::Home] and [Route::Blog] in a common navbar. The contents of the Home and Blog
/// routes will be rendered under the outlet inside this component
#[component]
pub fn Navbar() -> Element {
    let mut auth_user = use_resource(crate::models::get_current_user);

    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        nav {
            id: "navbar",
            class: "flex items-center justify-between p-4 bg-gray-900 text-white",
            Link {
                to: Route::Home {},
                class: "text-xl font-bold",
                "Taian CV"
            }

            div { class: "flex items-center gap-4",
                match auth_user.cloned() {
                    Some(Ok(Some(user))) => rsx! {
                        Link {
                            to: Route::Profile {},
                            class: "hover:text-blue-400 transition-colors flex items-center gap-2",
                            i { class: "fas fa-user-circle" }
                            "{user.email}"
                        }
                        button {
                            class: "text-sm bg-gray-800 hover:bg-gray-700 px-3 py-1 rounded transition-colors",
                            onclick: move |_| {
                                spawn(async move {
                                    let _ = crate::models::logout().await;
                                    auth_user.restart();
                                });
                            },
                            "Logout"
                        }
                    },
                    _ => rsx! {
                        button {
                            class: "bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg flex items-center gap-2 transition-colors",
                            onclick: move |_| {
                                spawn(async move {
                                    let _ = crate::models::login_mock().await;
                                    auth_user.restart();
                                });
                            },
                            i { class: "fab fa-google" }
                            "Login with Google"
                        }
                    }
                }
            }
        }

        Outlet::<Route> {}
    }
}
