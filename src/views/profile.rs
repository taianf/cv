use crate::components::{InfoField, LoginCard, SectionCard};
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
                        SectionCard {
                            title: "Personal Information".to_string(),
                            icon: "fa-id-card".to_string(),
                            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                                InfoField {
                                    label: "Email Address".to_string(),
                                    value: user.email.clone(),
                                    is_email: true
                                }
                                InfoField {
                                    label: "Account Type".to_string(),
                                    value: "Elite Member".to_string(),
                                    is_email: false
                                }
                            }
                        }

                        SectionCard {
                            title: "Security".to_string(),
                            icon: "fa-shield-halved".to_string(),
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
                LoginCard {
                    on_login: move |_| {
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
}
