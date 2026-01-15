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
        div { class: "flex items-center min-h-screen px-12 py-20",
            div { class: "grid grid-cols-1 lg:grid-cols-2 gap-16 items-center max-w-6xl",
                // Left side: Text
                div {
                    p { class: "text-blue-500 font-bold tracking-widest uppercase mb-4 text-sm",
                        "Fullstack Developer & AI Engineer"
                    }
                    h1 { class: "text-7xl font-black mb-8 leading-tight",
                        span { class: "block", "Taian" }
                        span { class: "block text-white/90", "Feitosa" }
                    }
                    p { class: "text-xl text-gray-400 mb-12 max-w-lg leading-relaxed",
                        "Building bridges between logic and creativity through code, "
                        "community, and innovative AI solutions."
                    }

                    div { class: "flex gap-8 items-center",
                        Link {
                            to: crate::Route::Profile {},
                            class: "group flex items-center gap-2 text-blue-500 font-bold hover:text-white transition-colors text-lg",
                            "About me"
                            i { class: "fas fa-arrow-right transition-transform group-hover:translate-x-1" }
                        }
                        a {
                            href: "https://github.com/taianf",
                            target: "_blank",
                            class: "group flex items-center gap-2 text-blue-500 font-bold hover:text-white transition-colors text-lg",
                            "Github"
                            i { class: "fas fa-arrow-right transition-transform group-hover:translate-x-1" }
                        }
                    }
                }

                // Right side: Image
                div { class: "relative group",
                    // Decorative background box
                    div { class: "absolute -inset-4 bg-blue-500/10 rounded-[40px] border border-blue-500/20 transform rotate-3 transition-transform group-hover:rotate-0" }

                    // Main image container
                    div { class: "relative aspect-square rounded-[32px] overflow-hidden bg-blue-600 shadow-2xl transition-transform group-hover:scale-[1.02] duration-500",
                        if let Some(user) = github_user() {
                            img {
                                src: "{user.avatar_url}",
                                class: "w-full h-full object-cover",
                                alt: "Profile",
                            }
                        } else {
                            div { class: "w-full h-full flex items-center justify-center bg-gray-800",
                                div { class: "animate-pulse flex flex-col items-center",
                                    i { class: "fas fa-user text-6xl text-gray-700 mb-4" }
                                    div { class: "h-4 w-32 bg-gray-700 rounded" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
