use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn NavItem(
    to: Route,
    icon: String,
    label: String,
    is_active: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let base_class =
        "flex items-center gap-3 px-6 py-3 rounded-xl transition-all duration-200 group";
    let active_class = "bg-blue-600 text-white font-bold shadow-lg shadow-blue-500/20";
    let inactive_class = "text-gray-400 hover:text-white hover:bg-gray-800";

    rsx! {
        Link {
            to: to,
            class: format!("{} {}", base_class, if is_active { active_class } else { inactive_class }),
            onclick: onclick,
            i { class: "fas {icon} w-5 text-center" }
            "{label}"
        }
    }
}
