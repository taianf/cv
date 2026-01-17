use crate::components::{LoginButton, Logo, NavItem, UserProfileCard};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    let mut auth_user = use_context::<Signal<Option<crate::models::AuthUser>>>();
    let mut is_menu_open = use_signal(|| false);

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

    let current_route = use_route::<Route>();

    rsx! {
        div { id: "main-container", class: "flex min-h-screen bg-[#0f1116]",
            button {
                class: "md:hidden fixed top-4 left-4 z-50 p-3 bg-gray-900/90 text-white rounded-xl shadow-lg border border-gray-800 backdrop-blur-sm",
                onclick: move |_| is_menu_open.set(!is_menu_open()),
                i { class: "fas fa-bars text-xl" }
            }

            if is_menu_open() {
                div {
                    class: "fixed inset-0 bg-black/50 backdrop-blur-sm z-30 md:hidden animate-in fade-in duration-200",
                    onclick: move |_| is_menu_open.set(false)
                }
            }

            nav {
                class: format!(
                    "w-72 bg-[#0a0c10] border-r border-gray-800 flex flex-col fixed h-screen z-40 transition-transform duration-300 ease-in-out {}",
                    if is_menu_open() { "translate-x-0" } else { "-translate-x-full md:translate-x-0" }
                ),

                button {
                    class: "absolute top-4 right-4 p-2 text-gray-400 hover:text-white md:hidden",
                    onclick: move |_| is_menu_open.set(false),
                    i { class: "fas fa-times text-xl" }
                }

                Logo {}

                div { class: "flex-grow px-4 space-y-2",
                    NavItem {
                        to: Route::Home {},
                        icon: "fa-home".to_string(),
                        label: "Home".to_string(),
                        is_active: current_route == Route::Home {},
                        onclick: move |_| is_menu_open.set(false),
                    }
                    NavItem {
                        to: Route::Blog {},
                        icon: "fa-book".to_string(),
                        label: "Blog".to_string(),
                        is_active: current_route == Route::Blog {},
                        onclick: move |_| is_menu_open.set(false),
                    }
                    a {
                        href: "/#about",
                        class: "cursor-pointer flex items-center gap-3 px-6 py-3 rounded-xl transition-all duration-200 group text-gray-400 hover:text-white hover:bg-gray-800",
                        onclick: move |_| {
                            is_menu_open.set(false);
                            #[cfg(feature = "web")]
                            {
                                if let Some(window) = web_sys::window() {
                                    if let Some(document) = window.document() {
                                        if let Some(element) = document.get_element_by_id("about") {
                                            let options = web_sys::ScrollIntoViewOptions::new();
                                            options.set_behavior(web_sys::ScrollBehavior::Smooth);
                                            element.scroll_into_view_with_scroll_into_view_options(&options);
                                        } else {
                                            let _ = window.location().set_href("/#about");
                                        }
                                    }
                                }
                            }
                        },
                        i { class: "fas fa-user w-5 text-center" }
                        "About"
                    }
                }

                div { class: "p-4 border-t border-gray-800",
                    if let Some(user) = auth_user() {
                        UserProfileCard {
                            email: user.email,
                            on_logout: move |_| {
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
                        }
                    } else {
                        LoginButton {
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
                        }
                    }
                }
            }

            main { class: "flex-grow md:ml-72 min-h-screen transition-all duration-300",
                Outlet::<Route> {}
            }
        }
    }
}
