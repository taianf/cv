use crate::Route;
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar() -> Element {
    let mut auth_user = use_context::<Signal<Option<crate::models::AuthUser>>>();

    // Load from local storage on first render
    use_effect(move || {
        #[cfg(feature = "web")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(email)) = storage.get_item("auth_email") {
                        auth_user.set(Some(crate::models::AuthUser { email }));
                    }
                }
            }
        }
    });

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
                if let Some(user) = auth_user() {
                    Link {
                        to: Route::Profile {},
                        class: "hover:text-blue-400 transition-colors flex items-center gap-2",
                        i { class: "fas fa-user-circle" }
                        "{user.email}"
                    }
                    button {
                        class: "text-sm bg-gray-800 hover:bg-gray-700 px-3 py-1 rounded transition-colors",
                        onclick: move |_| {
                            auth_user.set(None);
                            #[cfg(feature = "web")]
                            {
                                if let Some(window) = web_sys::window() {
                                    if let Ok(Some(storage)) = window.local_storage() {
                                        let _ = storage.remove_item("auth_email");
                                    }
                                }
                            }
                        },
                        "Logout"
                    }
                } else {
                    button {
                        class: "bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg flex items-center gap-2 transition-colors",
                        onclick: move |_| {
                            spawn(async move {
                                if let Ok(url) = crate::models::get_google_auth_url().await {
                                    #[cfg(feature = "web")]
                                    {
                                        if let Some(window) = web_sys::window() {
                                            let _ = window.location().set_href(&url);
                                        }
                                    }
                                }
                            });
                        },
                        i { class: "fab fa-google" }
                        "Login with Google"
                    }
                }
            }
        }

        Outlet::<Route> {}
    }
}
