use dioxus::prelude::*;

#[component]
pub fn Profile() -> Element {
    let auth_user = use_context::<Signal<Option<crate::models::AuthUser>>>();

    match auth_user() {
        Some(user) => {
            rsx! {
                div { class: "p-8 max-w-4xl",
                    h1 { class: "text-4xl font-bold mb-8 text-blue-500", "Account Settings" }

                    div { class: "grid gap-8",
                        div { class: "bg-gray-800 p-8 rounded-2xl border border-gray-700",
                            h2 { class: "text-xl font-bold mb-6 flex items-center gap-2",
                                i { class: "fas fa-id-card text-blue-500" }
                                "Personal Information"
                            }

                            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                                div {
                                    label { class: "block text-xs uppercase text-gray-500 mb-1", "Email Address" }
                                    div { class: "bg-gray-900 p-3 rounded-lg text-white font-medium border border-gray-800", "{user.email}" }
                                }
                                div {
                                    label { class: "block text-xs uppercase text-gray-500 mb-1", "Account Type" }
                                    div { class: "bg-gray-900 p-3 rounded-lg text-blue-500 font-bold border border-blue-500/20", "Elite Member" }
                                }
                            }
                        }

                        div { class: "bg-gray-800 p-8 rounded-2xl border border-gray-700",
                            h2 { class: "text-xl font-bold mb-6 flex items-center gap-2",
                                i { class: "fas fa-shield-halved text-blue-500" }
                                "Security"
                            }
                            button { class: "bg-gray-900 hover:bg-gray-750 text-white px-4 py-2 rounded-lg border border-gray-700 transition-colors",
                                "Change Password"
                            }
                        }
                    }
                }
            }
        }
        None => {
            rsx! {
                div { class: "flex items-center justify-center min-h-[60vh]",
                    div { class: "text-center",
                        i { class: "fas fa-user-circle text-8xl text-gray-800 mb-6" }
                        h2 { class: "text-3xl font-bold text-white mb-4", "Not Authenticated" }
                        p { class: "text-gray-500 mb-8 max-w-xs mx-auto", "Join our community today to access personalized features and members-only content." }
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
}
