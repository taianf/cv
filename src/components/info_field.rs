use dioxus::prelude::*;

#[component]
pub fn InfoField(label: String, value: String, is_email: bool) -> Element {
    rsx! {
        div {
            label { class: "block text-xs uppercase text-gray-500 mb-1", "{label}" }
            div {
                class: format!(
                    "p-3 rounded-lg border {}",
                    if is_email {
                        "bg-gray-900 text-white font-medium border-gray-800"
                    } else {
                        "bg-gray-900 text-blue-500 font-bold border-blue-500/20"
                    }
                ),
                "{value}"
            }
        }
    }
}
