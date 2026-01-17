use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn UserProfileCard(email: String, on_logout: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div { class: "p-4 bg-gray-900/50 rounded-2xl border border-gray-800",
            div { class: "flex items-center gap-3 mb-3",
                div { class: "w-10 h-10 bg-blue-500 rounded-full flex items-center justify-center text-white font-bold",
                    "{email.chars().next().unwrap_or('?').to_uppercase()}"
                }
                div { class: "overflow-hidden",
                    p { class: "text-sm font-bold text-white truncate", "{email}" }
                    p { class: "text-xs text-gray-500", "Member Status: Pro" }
                }
            }
            Link {
                to: Route::Profile {},
                class: "block w-full text-center py-2 text-sm text-blue-500 hover:text-blue-400 font-medium transition-colors",
                "Manage Account"
            }
            button {
                class: "block w-full text-center py-1 text-xs text-gray-600 hover:text-red-400 font-medium transition-colors mt-1",
                onclick: on_logout,
                "Logout"
            }
        }
    }
}
