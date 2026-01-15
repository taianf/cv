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
        div { class: "w-full",
            // Hero Section
            div { class: "flex items-center min-h-[90vh] px-12 py-20",
                div { class: "grid grid-cols-1 lg:grid-cols-2 gap-16 items-center max-w-6xl mx-auto",
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
                            a {
                                href: "https://github.com/taianf",
                                target: "_blank",
                                class: "group flex items-center gap-2 text-blue-500 font-bold hover:text-white transition-colors text-lg",
                                i { class: "fab fa-github text-xl" }
                                "Github"
                            }
                            a {
                                href: "https://www.linkedin.com/in/taian-feitosa/",
                                target: "_blank",
                                class: "group flex items-center gap-2 text-blue-500 font-bold hover:text-white transition-colors text-lg",
                                i { class: "fab fa-linkedin text-xl" }
                                "LinkedIn"
                            }
                        }
                    }

                    // Right side: Image
                    div { class: "relative group",
                        // Decorative background box
                        div { class: "absolute -inset-4 bg-blue-500/10 rounded-[40px] border border-blue-500/20 transform rotate-3 transition-transform group-hover:rotate-0" }

                        // Main image container
                        div { class: "relative aspect-square rounded-[32px] overflow-hidden bg-blue-600 shadow-2xl transition-transform group-hover:scale-[1.02] duration-500",
                            img {
                                src: "https://media.licdn.com/dms/image/v2/D4D03AQFvFMptVJnakQ/profile-displayphoto-shrink_800_800/profile-displayphoto-shrink_800_800/0/1673901553883?e=1770249600&v=beta&t=KYANNIIT41GRnj54wCutvAhjrRTF3JXdKGNU1OpymSc",
                                class: "w-full h-full object-cover",
                                alt: "Taian Feitosa",
                            }
                        }
                    }
                }
            }


            // About Section
            div { id: "about", class: "px-12 py-20",
                div { class: "max-w-4xl mx-auto",
                    h2 { class: "text-3xl font-bold mb-12 flex items-center gap-4",
                        span { class: "w-12 h-1 bg-blue-500 rounded-full" }
                        "About me"
                    }

                    div { class: "prose prose-invert prose-lg text-gray-400 leading-relaxed",
                        p { class: "mb-6",
                            "I am a Fullstack Developer and AI Engineer passionate about building bridges between "
                            "users and technology. With a deep focus on Rust and modern web frameworks like Dioxus, "
                            "I strive to create performant and accessible applications."
                        }
                        p { class: "mb-6",
                            "My journey in tech is driven by curiosity and community. I believe in open source, "
                            "knowledge sharing, and the power of software to solve real-world problems. "
                            "When I'm not coding, I'm exploring new AI models or contributing to the developer ecosystem."
                        }
                        p {
                            "Feel free to reach out via my social links or join our community forum to start a conversation!"
                        }
                    }
                }
            }
        }
    }
}
