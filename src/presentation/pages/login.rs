use dioxus::prelude::*;
use crate::data::SaveManager;
use crate::models::Route;
use crate::business_logic::{TodoNodeFactory, SaveDataBuilder};

#[component]
pub fn Login() -> Element {
    let mut text = use_signal(|| String::new());
    let mut message = use_signal(|| String::new());
    let mut seed_signal: Signal<Option<String>> = use_context();
    let mut language_signal: Signal<String> = use_context();
    let mut current_route: Signal<Route> = use_context();

    rsx! {
        div { class: "flex flex-col items-center justify-center h-screen bg-gray-800 gap-4",

            h1 { class: "text-4xl font-bold text-blue-200", "Enter Your Seed" }

            input {
                class: "border border-gray-300 rounded px-4 py-2",
                placeholder: "Type any seed...",
                value: "{text}",
                oninput: move |e| text.set(e.value()),
                onkeydown: move |e| {
                    if e.key() == Key::Enter {
                        if process_login(
                            &text(),
                            &mut seed_signal,
                            &mut language_signal,
                            &mut message,
                        ) {
                            current_route.set(Route::Home);
                        }
                    }
                },
            }

            button {
                class: "mt-4 bg-blue-500 text-white px-6 py-2 rounded hover:bg-blue-600",
                onclick: move |_: MouseEvent| {
                    if process_login(&text(), &mut seed_signal, &mut language_signal, &mut message) {
                        current_route.set(Route::Home);
                    }
                },
                "Enter"
            }

            p { class: "text-white", "{message}" }
        }
    }
}

fn process_login(
    seed: &str,
    seed_signal: &mut Signal<Option<String>>,
    language_signal: &mut Signal<String>,
    message: &mut Signal<String>,
) -> bool {
    if seed.is_empty() {
        message.set("❌ Please enter a seed.".to_string());
        return false;
    }

    let save_data = SaveManager::load_save(seed);

    match save_data {
        Some(data) => {
            seed_signal.set(Some(seed.to_string()));
            language_signal.set(data.language);
        }
        None => {
            seed_signal.set(Some(seed.to_string()));
            
            // Build initial data using Factory & Builder patterns
            let learn_rust = TodoNodeFactory::create_regular(0, "Learn Rust".to_string());
            let subtask1 = TodoNodeFactory::create_subtask(1, "New Subtask".to_string(), 1);
            let subtask2 = TodoNodeFactory::create_subtask(2, "New Subtask".to_string(), 1);
            
            let mut root_task = learn_rust;
            root_task.children = vec![subtask1, subtask2];
            
            let new_save = SaveDataBuilder::new(seed.to_string())
                .with_language("English".to_string())
                .with_initial_task(root_task)
                .with_next_id(3)
                .with_points(0)
                .build();
            
            SaveManager::save_data(&new_save);
            language_signal.set("English".to_string());
        }
    }
    true
}
