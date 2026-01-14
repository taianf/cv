use crate::models::GitHubUser;
use dioxus::prelude::*;

#[component]
pub fn ProfileHeader(user: Option<GitHubUser>) -> Element {
    rsx! {
        header { class: "text-center mb-12",
            if let Some(user) = user {
                img {
                    src: "{user.avatar_url}",
                    class: "w-32 h-32 rounded-full mx-auto mb-4 border-4 border-blue-500 shadow-2xl"
                }
            }
            h1 { class: "text-5xl font-extrabold text-white mb-4", "Taian" }
            p { class: "text-xl text-gray-400", "Software Engineer | Rust Enthusiast | Web Developer" }
        }
    }
}
