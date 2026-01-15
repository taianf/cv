use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    let mut auth_user = use_context::<Signal<Option<crate::models::AuthUser>>>();
    let mut is_menu_open = use_signal(|| false);

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

    let current_route = use_route::<Route>();

    let nav_item_class = |route: Route| {
        let base = "flex items-center gap-3 px-6 py-3 rounded-xl transition-all duration-200 group";
        if current_route == route {
            format!(
                "{} bg-blue-600 text-white font-bold shadow-lg shadow-blue-500/20",
                base
            )
        } else {
            format!("{} text-gray-400 hover:text-white hover:bg-gray-800", base)
        }
    };

    rsx! {
        div { id: "main-container", class: "flex min-h-screen bg-[#0f1116]",
            // Mobile Hamburger Button
            button {
                class: "md:hidden fixed top-4 left-4 z-50 p-3 bg-gray-900/90 text-white rounded-xl shadow-lg border border-gray-800 backdrop-blur-sm",
                onclick: move |_| is_menu_open.set(!is_menu_open()),
                i { class: "fas fa-bars text-xl" }
            }

            // Mobile Backdrop
            if is_menu_open() {
                div {
                    class: "fixed inset-0 bg-black/50 backdrop-blur-sm z-30 md:hidden animate-in fade-in duration-200",
                    onclick: move |_| is_menu_open.set(false)
                }
            }

            // Sidebar
            nav {
                class: format!(
                    "w-72 bg-[#0a0c10] border-r border-gray-800 flex flex-col fixed h-screen z-40 transition-transform duration-300 ease-in-out {}",
                    if is_menu_open() { "translate-x-0" } else { "-translate-x-full md:translate-x-0" }
                ),

                // Close Button (Mobile only)
                button {
                    class: "absolute top-4 right-4 p-2 text-gray-400 hover:text-white md:hidden",
                    onclick: move |_| is_menu_open.set(false),
                    i { class: "fas fa-times text-xl" }
                }

                // Logo
                div { class: "p-8 mb-4",
                    h1 { class: "text-2xl font-black tracking-tighter uppercase italic",
                        span { class: "text-white", "TAIAN " }
                        span { class: "text-blue-500", "FEITOSA" }
                    }
                }

                // Nav Links
                div { class: "flex-grow px-4 space-y-2",
                    Link {
                        to: Route::Home {},
                        class: nav_item_class(Route::Home {}),
                        onclick: move |_| is_menu_open.set(false),
                        i { class: "fas fa-home w-5 text-center" }
                        "Home"
                    }
                    Link {
                        to: Route::Blog {},
                        class: nav_item_class(Route::Blog {}),
                        onclick: move |_| is_menu_open.set(false),
                        i { class: "fas fa-book w-5 text-center" }
                        "Blog"
                    }
                    // About link with smooth scroll handling
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
                                            element.scroll_into_view_with_scroll_into_view_options(
                                                web_sys::ScrollIntoViewOptions::new().behavior(web_sys::ScrollBehavior::Smooth)
                                            );
                                        } else {
                                            // improved navigation logic: if strictly on a different route, navigate first
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

                // Profile / Login at bottom
                div { class: "p-4 border-t border-gray-800",
                    if let Some(user) = auth_user() {
                        div { class: "p-4 bg-gray-900/50 rounded-2xl border border-gray-800",
                            div { class: "flex items-center gap-3 mb-3",
                                div { class: "w-10 h-10 bg-blue-500 rounded-full flex items-center justify-center text-white font-bold",
                                    "{user.email.chars().next().unwrap_or('?').to_uppercase()}"
                                }
                                div { class: "overflow-hidden",
                                    p { class: "text-sm font-bold text-white truncate", "{user.email}" }
                                    p { class: "text-xs text-gray-500", "Member Status: Pro" }
                                }
                            }
                            Link {
                                to: Route::Profile {},
                                class: "block w-full text-center py-2 text-sm text-blue-500 hover:text-blue-400 font-medium transition-colors",
                                onclick: move |_| is_menu_open.set(false),
                                "Manage Account"
                            }
                            button {
                                class: "block w-full text-center py-1 text-xs text-gray-600 hover:text-red-400 font-medium transition-colors mt-1",
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
                        }
                    } else {
                        button {
                            class: "w-full bg-blue-600 hover:bg-blue-700 text-white font-bold py-3 px-6 rounded-xl flex items-center justify-center gap-2 transition-all shadow-lg shadow-blue-500/10",
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
                            i { class: "fas fa-sign-in-alt" }
                            "Login"
                        }
                    }
                }
            }

            // Main Content Area
            main { class: "flex-grow md:ml-72 min-h-screen transition-all duration-300",
                Outlet::<Route> {}
            }
        }
    }
}
