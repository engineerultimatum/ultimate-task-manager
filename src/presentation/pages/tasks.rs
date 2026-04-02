use dioxus::prelude::*;
use crate::data::SaveManager;
use crate::models::{Route, TaskType};
use crate::business_logic::{add_root_node_with_type, reset_habit_if_needed};
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
                let mut tasks = save.todos;
                reset_habit_if_needed(&mut tasks);
                return tasks;
            }
        }
        vec![]
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

    let mut show_task_type_modal = use_signal(|| false);

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
                    show_task_type_modal.set(true);
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

            if show_task_type_modal() {
                TaskTypeModal {
                    on_select: move |task_type: TaskType| {
                        add_root_node_with_type(todos.write().as_mut(), next_id(), task_type);
                        next_id += 1;
                        show_task_type_modal.set(false);
                    },
                    on_cancel: move |_| {
                        show_task_type_modal.set(false);
                    },
                }
            }
        }
    }
}

#[component]
fn TaskTypeModal(
    on_select: EventHandler<TaskType>,
    on_cancel: EventHandler<()>,
) -> Element {
    rsx! {
        div { class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
            div { class: "bg-blue-900 rounded-lg shadow-lg p-6 w-96",
                h2 { class: "text-xl font-bold mb-4 text-white", "Select Task Type" }
                div { class: "flex flex-col gap-3 mb-6",
                    button {
                        class: "px-4 py-3 bg-green-600 text-white rounded hover:bg-green-500 font-semibold",
                        onclick: move |_| on_select.call(TaskType::Regular),
                        "📋 Regular Task"
                    }
                    button {
                        class: "px-4 py-3 bg-purple-600 text-white rounded hover:bg-purple-500 font-semibold",
                        onclick: move |_| on_select.call(TaskType::Habit),
                        "🔄 Daily Habit"
                    }
                }
                button {
                    class: "w-full px-4 py-2 bg-gray-600 text-white rounded hover:bg-gray-700",
                    onclick: move |_| on_cancel.call(()),
                    "Cancel"
                }
            }
        }
    }
}
