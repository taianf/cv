use dioxus::prelude::*;

#[component]
pub fn Members() -> Element {
    let auth_user = use_context::<Signal<Option<crate::models::AuthUser>>>();

    rsx! {
        div { class: "p-8",
            h1 { class: "text-4xl font-bold mb-8 text-blue-500", "Members Area" }

            if let Some(user) = auth_user() {
                div { class: "bg-gray-800 p-8 rounded-2xl border border-blue-500/30",
                    div { class: "flex items-center gap-4 mb-6",
                        div { class: "w-16 h-16 bg-blue-600 rounded-full flex items-center justify-center text-white text-2xl font-bold",
                            "{user.email.chars().next().unwrap_or('?').to_uppercase()}"
                        }
                        div {
                            h2 { class: "text-2xl font-bold", "Welcome back!" }
                            p { class: "text-gray-400", "{user.email}" }
                        }
                    }

                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                        for card in ["Exclusive Content", "Priority Support", "Community Badges", "Early Access"] {
                            div { class: "bg-gray-900/50 p-4 rounded-lg border border-gray-700",
                                h3 { class: "font-semibold mb-1", "{card}" }
                                p { class: "text-sm text-gray-500", "Access is granted to your account." }
                            }
                        }
                    }
                }
            } else {
                div { class: "bg-gray-800 p-8 rounded-2xl border border-gray-700 text-center",
                    i { class: "fas fa-lock text-5xl text-blue-500 mb-4" }
                    h2 { class: "text-2xl font-bold mb-2", "Members Only" }
                    p { class: "text-gray-400 mb-6", "Please log in to access the exclusive members area." }
                    button {
                        class: "bg-blue-600 hover:bg-blue-700 text-white font-bold py-3 px-8 rounded-full transition-all",
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
                        "Login with Google"
                    }
                }
            }
        }
    }
}
