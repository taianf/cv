use crate::components::{GitHubCard, LinkedInCard, ProfileHeader, SocialLinks};
use crate::models::{get_github_user, GitHubUser};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let mut github_user = use_signal(|| None::<GitHubUser>);

    use_resource(move || async move {
        if let Ok(user) = get_github_user("taianf".to_string()).await {
            github_user.set(Some(user));
        }
    });

    rsx! {
        div { class: "max-w-4xl mx-auto py-12 px-6",
            ProfileHeader { user: github_user() }

            section { class: "grid grid-cols-1 md:grid-cols-2 gap-8",
                GitHubCard { user: github_user() }
                LinkedInCard {}
            }

            SocialLinks {}
        }
    }
}
