use dioxus::prelude::*;
use crate::data::SaveManager;
use crate::models::{Route, TodoNode};
use crate::business_logic::add_root_node;
use crate::presentation::components::TreeNode;

#[component]
pub fn Go() -> Element {
    let seed_signal: Signal<Option<String>> = use_context();
    let language_signal: Signal<String> = use_context();
    let mut current_route: Signal<Route> = use_context();
    
    // Load initial data from save
    let mut todos = use_signal(|| {
        if let Some(seed) = seed_signal.read().as_ref() {
            if let Some(save) = SaveManager::load_save(seed) {
                return save.todos;
            }
        }
        vec![
            TodoNode {
                id: 0,
                text: "Learn Rust".to_string(),
                completed: false,
                importance: 1,
                children: vec![
                    TodoNode { id: 1, text: "New Subtask".to_string(), completed: false, importance: 1, children: vec![], deadline: None },
                    TodoNode { id: 2, text: "New Subtask".to_string(), completed: false, importance: 1, children: vec![], deadline: None },
                ],
                deadline: None,
            },
        ]
    });
    
    let mut next_id = use_signal(|| {
        if let Some(seed) = seed_signal.read().as_ref() {
            if let Some(save) = SaveManager::load_save(seed) {
                return save.next_id;
            }
        }
        3
    });
    
    let points = use_signal(|| {
        if let Some(seed) = seed_signal.read().as_ref() {
            if let Some(save) = SaveManager::load_save(seed) {
                return save.points;
            }
        }
        0u32
    });

    // Auto-save whenever todos or points change
    use_effect(move || {
        if let Some(seed) = seed_signal.read().as_ref() {
            let save = crate::models::SaveData {
                seed: seed.clone(),
                language: language_signal.read().clone(),
                todos: todos.read().clone(),
                next_id: next_id.read().clone(),
                points: points.read().clone(),
            };
            SaveManager::save_data(&save);
        }
    });

    rsx! {
        div { class: "container mx-auto p-8 bg-black text-white min-h-screen",
            div { class: "relative flex justify-between items-center mb-6",
                h1 { class: "text-3xl font-bold", "UTiM" }
                div { class: "text-2xl font-bold text-yellow-400", "Points: {points}" }
                h2 {
                    class: "absolute right-8 top-0 text-gray-400 hover:text-gray-200 cursor-pointer",
                    onclick: move |_| {
                        current_route.set(Route::Home);
                    },
                    if language_signal() == "English" {
                        "Back"
                    } else {
                        "Geri"
                    }
                }
            }
            button {
                class: "text-lg text-green-500 mb-4 px-4 py-2 bg-green-700 rounded hover:bg-green-600",
                onclick: move |_| {
                    add_root_node(todos.write().as_mut(), next_id());
                    next_id += 1;
                },
                "+ Add Root Task"
            }

            ul { class: "space-y-2",
                for node in todos.read().iter() {
                    TreeNode {
                        node: node.clone(),
                        todos,
                        next_id,
                        points,
                    }
                }
            }
        }
    }
}
