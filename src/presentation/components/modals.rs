use dioxus::prelude::*;
use crate::presentation::components::ImportanceSelector;
use crate::business_logic::format_month;
use crate::models::SaveData;
use crate::data::SaveManager;

#[component]
pub fn EditModal(
    initial_text: String,
    initial_importance: u32,
    on_rename: EventHandler<(String, u32)>,
    on_cancel: EventHandler<()>,
) -> Element {
    let mut input_value = use_signal(|| initial_text.clone());
    let importance = use_signal(|| initial_importance);

    rsx! {
        div { class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
            div { class: "bg-blue-900 rounded-lg shadow-lg p-6 w-96",
                h2 { class: "text-xl font-bold mb-4 text-white", "Rename Task" }
                input {
                    class: "w-full px-4 py-2 border rounded mb-4",
                    value: "{input_value}",
                    oninput: move |e| input_value.set(e.value()),
                    autofocus: true,
                    onkeydown: move |e| {
                        if e.key() == Key::Enter {
                            on_rename.call((input_value(), importance()));
                        }
                    },
                }
                ImportanceSelector { importance }
                div { class: "flex gap-2 justify-end",
                    button {
                        class: "px-4 py-2 bg-gray-600 text-white rounded hover:bg-gray-700",
                        onclick: move |_| on_cancel.call(()),
                        "Cancel"
                    }
                    button {
                        class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600",
                        onclick: move |_| on_rename.call((input_value(), importance())),
                        "Rename"
                    }
                }
            }
        }
    }
}

#[component]
pub fn CompletionModal(
    task_name: String,
    on_completed: EventHandler<()>,
    on_cancel: EventHandler<()>,
) -> Element {
    rsx! {
        div { class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
            div { class: "bg-blue-900 rounded-lg shadow-lg p-6 w-96",
                h2 { class: "text-xl font-bold mb-4 text-white", "Complete Task?" }
                p { class: "text-white mb-6", "Mark '{task_name}' as completed?" }
                div { class: "flex gap-2 justify-end",
                    button {
                        class: "px-4 py-2 bg-gray-600 text-white rounded hover:bg-gray-700",
                        onclick: move |_| on_cancel.call(()),
                        "Cancel"
                    }
                    button {
                        class: "px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600",
                        onclick: move |_| on_completed.call(()),
                        "Completed"
                    }
                }
            }
        }
    }
}

#[component]
pub fn DayModal(
    day: i32,
    month: u32,
    year: u32,
    todos: Signal<Vec<crate::models::TodoNode>>,
    mut next_id: Signal<usize>,
    seed_signal: Signal<Option<String>>,
    language_signal: Signal<String>,
    on_create: EventHandler<(String, u32)>,
    on_cancel: EventHandler<()>,
) -> Element {
    let mut task_name = use_signal(|| String::new());
    let mut importance = use_signal(|| 1);

    rsx! {
        div { class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
            div { class: "bg-blue-900 rounded-lg shadow-lg p-6 w-96",
                h2 { class: "text-xl font-bold mb-4 text-white",
                    "Add Task for {day} {format_month(month as i32)}"
                }
                input {
                    class: "w-full px-4 py-2 border rounded mb-4 bg-gray-800 text-white border-gray-600",
                    placeholder: "Task name...",
                    value: "{task_name}",
                    oninput: move |e| task_name.set(e.value()),
                    autofocus: true,
                    onkeydown: move |e| {
                        if e.key() == Key::Enter && !task_name().is_empty() {
                            on_create.call((task_name(), importance()));
                            task_name.set(String::new());
                        }
                    },
                }
                ImportanceSelector { importance }
                div { class: "flex gap-2 justify-end",
                    button {
                        class: "px-4 py-2 bg-gray-600 text-white rounded hover:bg-gray-700",
                        onclick: move |_| {
                            on_cancel.call(());
                            task_name.set(String::new());
                        },
                        "Cancel"
                    }
                    button {
                        class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 disabled:opacity-50",
                        disabled: task_name().is_empty(),
                        onclick: move |_| {
                            if !task_name().is_empty() {
                                on_create.call((task_name(), importance()));
                                task_name.set(String::new());
                            }
                        },
                        "Create Task"
                    }
                }
            }
        }
    }
}
