use dioxus::prelude::*;

#[component]
pub fn Profile() -> Element {
    let auth_user = use_resource(crate::models::get_current_user);

    match auth_user.cloned() {
        Some(Ok(Some(user))) => {
            rsx! {
                div { class: "max-w-2xl mx-auto py-12 px-6",
                    div { class: "bg-gray-800 p-8 rounded-2xl shadow-xl border border-gray-700",
                        h2 { class: "text-3xl font-bold text-white mb-6 flex items-center gap-3",
                            i { class: "fas fa-user-circle text-blue-500" }
                            "User Profile"
                        }

                        div { class: "space-y-6",
                            div { class: "bg-gray-700/50 p-4 rounded-lg",
                                span { class: "block text-sm text-gray-400 uppercase tracking-wider mb-1", "Username / Email" }
                                span { class: "text-xl text-white font-medium", "{user.email}" }
                            }

                            div { class: "pt-4 border-t border-gray-700",
                                p { class: "text-gray-400 text-sm italic",
                                    "This information was retrieved from your Google Account."
                                }
                            }
                        }
                    }
                }
            }
        }
        Some(Ok(None)) => {
            rsx! {
                div { class: "text-center py-20",
                    h2 { class: "text-2xl text-white", "Please log in to view your profile." }
                }
            }
        }
        _ => {
            rsx! {
                div { class: "text-center py-20 text-gray-400 animate-pulse", "Checking authentication..." }
            }
        }
    }
}
