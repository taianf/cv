use dioxus::prelude::*;

#[component]
pub fn LoginCard(on_login: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div { class: "flex items-center justify-center min-h-[60vh]",
            div { class: "text-center",
                i { class: "fas fa-user-circle text-8xl text-gray-800 mb-6" }
                h2 { class: "text-3xl font-bold text-white mb-4", "Not Authenticated" }
                p { class: "text-gray-500 mb-8 max-w-xs mx-auto", "Join our community today to access personalized features and members-only content." }
                button {
                    class: "bg-blue-600 hover:bg-blue-700 text-white font-bold py-3 px-8 rounded-full transition-all",
                    onclick: on_login,
                    "Login with Google"
                }
            }
        }
    }
}
