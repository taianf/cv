use crate::models::GitHubUser;
use dioxus::prelude::*;

#[component]
pub fn GitHubCard(user: Option<GitHubUser>) -> Element {
    rsx! {
        div { class: "bg-gray-800 p-6 rounded-2xl shadow-xl border border-gray-700 hover:border-blue-500 transition-colors",
            h2 { class: "text-2xl font-bold text-white mb-4 flex items-center gap-2",
                i { class: "fab fa-github" }
                "GitHub"
            }
            div { class: "space-y-4",
                if let Some(user) = user.as_ref() {
                    div { class: "grid grid-cols-2 gap-4 text-center",
                        div { class: "bg-gray-700 p-3 rounded-lg",
                            span { class: "block text-2xl font-bold text-blue-400",
                                "{user.public_repos}"
                            }
                            span { class: "text-xs text-gray-400 uppercase", "Repos" }
                        }
                        div { class: "bg-gray-700 p-3 rounded-lg",
                            span { class: "block text-2xl font-bold text-green-400",
                                "{user.followers}"
                            }
                            span { class: "text-xs text-gray-400 uppercase", "Followers" }
                        }
                    }
                    if let Some(bio) = &user.bio {
                        p { class: "text-gray-300 italic text-sm", "\"{bio}\"" }
                    }
                } else {
                    p { class: "text-gray-300 animate-pulse", "Fetching real-time stats..." }
                }
                a {
                    href: if let Some(user) = user.as_ref() { format!("https://github.com/{}", user.login) } else { "https://github.com/taianf".to_string() },
                    target: "_blank",
                    rel: "noopener noreferrer",
                    class: "inline-block w-full text-center bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-4 rounded-lg transition-colors",
                    "View Profile"
                }
            }
        }
    }
}
