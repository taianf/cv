use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    let mut auth_user = use_context::<Signal<Option<crate::models::AuthUser>>>();
    let mut show_notifications = use_signal(|| false);
    let mut notifications = use_signal(|| {
        vec![
            "New post in the forum!",
            "Someone liked your profile",
            "System maintenance at 2 AM",
        ]
    });

    // Load status from local storage on first render
    use_effect(move || {
        #[cfg(feature = "web")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    // Profile
                    if let Ok(Some(email)) = storage.get_item("auth_email") {
                        auth_user.set(Some(crate::models::AuthUser { email }));
                    }

                    // Notifications Status
                    if let Ok(Some(status)) = storage.get_item("notifications_cleared") {
                        if status == "true" {
                            notifications.write().clear();
                        }
                    }
                }
            }
        }
    });

    // Save notification status whenever it changes
    use_effect(move || {
        let is_empty = notifications.read().is_empty();
        #[cfg(feature = "web")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.set_item(
                        "notifications_cleared",
                        if is_empty { "true" } else { "false" },
                    );
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
            // Sidebar
            nav {
                class: "w-72 bg-[#0a0c10] border-r border-gray-800 flex flex-col fixed h-screen z-20",

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
                        i { class: "fas fa-home w-5 text-center" }
                        "Início"
                    }
                    Link {
                        to: Route::Blog {},
                        class: nav_item_class(Route::Blog {}),
                        i { class: "fas fa-book w-5 text-center" }
                        "Blog"
                    }
                    Link {
                        to: Route::Members {},
                        class: nav_item_class(Route::Members {}),
                        i { class: "fas fa-crown w-5 text-center" }
                        "Members Area"
                    }
                    Link {
                        to: Route::Forum {},
                        class: nav_item_class(Route::Forum {}),
                        i { class: "fas fa-comments w-5 text-center" }
                        "Forum"
                    }

                    // Notification Button (Not a route, but a toggle)
                    button {
                        class: "w-full flex items-center justify-between gap-3 px-6 py-3 rounded-xl text-gray-400 hover:text-white hover:bg-gray-800 transition-all group",
                        onclick: move |_| show_notifications.set(!show_notifications()),
                        div { class: "flex items-center gap-3",
                            i { class: "fas fa-bell w-5 text-center" }
                            "Notifications"
                        }
                        if !notifications.read().is_empty() {
                            span { class: "bg-blue-500 text-white text-[10px] font-bold px-1.5 py-0.5 rounded-full",
                                "{notifications.read().len()}"
                            }
                        }
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
                            i { class: "fab fa-google" }
                            "Login"
                        }
                    }
                }
            }

            // Notification Overlay
            if show_notifications() {
                div {
                    class: "fixed inset-0 z-30",
                    onclick: move |_| show_notifications.set(false),
                    div {
                        class: "absolute left-76 top-20 w-80 bg-gray-900 border border-gray-700 rounded-2xl shadow-2xl p-4 animate-in fade-in slide-in-from-left-4 duration-200",
                        onclick: move |e| e.stop_propagation(),
                        div { class: "flex justify-between items-center mb-4 pb-2 border-b border-gray-800",
                            h4 { class: "font-bold text-white", "Notifications" }
                            button {
                                class: "text-xs text-blue-500",
                                onclick: move |_| notifications.write().clear(),
                                "Clear all"
                            }
                        }
                        div { class: "space-y-3",
                            if notifications.read().is_empty() {
                                p { class: "text-sm text-gray-500 text-center py-4", "No new notifications" }
                            } else {
                                for note in notifications.read().iter() {
                                    div { class: "flex gap-3 items-start p-2 hover:bg-gray-800 rounded-lg transition-colors cursor-pointer",
                                        div { class: "w-2 h-2 mt-1.5 rounded-full bg-blue-500 flex-shrink-0" }
                                        p { class: "text-sm text-gray-300", "{note}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Main Content Area
            main { class: "flex-grow ml-72 min-h-screen",
                Outlet::<Route> {}
            }
        }
    }
}
