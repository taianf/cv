use dioxus::prelude::*;

#[component]
pub fn SectionCard(title: String, icon: String, children: Element) -> Element {
    rsx! {
        div { class: "bg-gray-800 p-8 rounded-2xl border border-gray-700",
            h2 { class: "text-xl font-bold mb-6 flex items-center gap-2",
                i { class: "fas {icon} text-blue-500" }
                "{title}"
            }
            {children}
        }
    }
}
