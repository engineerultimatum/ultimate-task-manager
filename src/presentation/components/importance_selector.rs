use dioxus::prelude::*;

#[component]
pub fn ImportanceSelector(mut importance: Signal<u32>) -> Element {
    rsx! {
        div { class: "mb-4",
            label { class: "block text-white mb-2", "Importance" }
            div { class: "flex gap-2",
                button {
                    class: if importance() == 1 { "px-4 py-2 bg-green-500 text-white rounded font-bold" } else { "px-4 py-2 bg-green-700 text-white rounded hover:bg-green-600" },
                    onclick: move |_| importance.set(1),
                    "🟢 Low"
                }
                button {
                    class: if importance() == 2 { "px-4 py-2 bg-red-500 text-white rounded font-bold" } else { "px-4 py-2 bg-red-700 text-white rounded hover:bg-red-600" },
                    onclick: move |_| importance.set(2),
                    "🔴 Medium"
                }
                button {
                    class: if importance() == 3 { "px-4 py-2 bg-purple-500 text-white rounded font-bold" } else { "px-4 py-2 bg-purple-700 text-white rounded hover:bg-purple-600" },
                    onclick: move |_| importance.set(3),
                    "🟣 High"
                }
            }
        }
    }
}
