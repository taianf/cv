use dioxus::prelude::*;

#[component]
pub fn LoadingSpinner(message: String) -> Element {
    rsx! {
        div { class: "flex items-center justify-center min-h-screen bg-gray-900",
            div { class: "text-center",
                div { class: "animate-spin rounded-full h-16 w-16 border-t-2 border-b-2 border-blue-500 mx-auto mb-4" }
                p { class: "text-xl text-gray-300", "{message}" }
            }
        }
    }
}
