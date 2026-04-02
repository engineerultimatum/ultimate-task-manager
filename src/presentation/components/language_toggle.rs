use dioxus::prelude::*;

#[component]
pub fn LanguageToggle(mut language: Signal<String>) -> Element {
    rsx! {
        div { class: "flex gap-2",

            button {
                class: if language() == "English" { "px-4 py-2 rounded bg-blue-600 text-white" } else { "px-4 py-2 rounded bg-zinc-700 text-gray-400 hover:bg-zinc-600" },
                onclick: move |_| language.set("English".to_string()),
                if language() == "English" {
                    h1 { "English" }
                } else {
                    h1 { "İngilizce" }
                }
            }

            button {
                class: if language() == "Turkish" { "px-4 py-2 rounded bg-blue-600 text-white" } else { "px-4 py-2 rounded bg-zinc-700 text-gray-400 hover:bg-zinc-600" },
                onclick: move |_| language.set("Turkish".to_string()),
                if language() == "English" {
                    h1 { "Turkish" }
                } else {
                    h1 { "Türkçe" }
                }
            }
        }
    }
}
