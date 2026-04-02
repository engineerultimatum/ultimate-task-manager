use dioxus::prelude::*;
use crate::models::TodoNode;
use crate::business_logic::{rename_node, delete_node, add_child, calculate_importance_points};
use crate::presentation::components::{EditModal, CompletionModal, ImportanceSelector};

#[component]
pub fn TreeNode(node: TodoNode, todos: Signal<Vec<TodoNode>>, mut next_id: Signal<usize>, mut points: Signal<u32>) -> Element {
    let mut expanded = use_signal(|| true);
    let mut edit_modal_open = use_signal(|| false);
    let mut completion_modal_open = use_signal(|| false);
    let mut edit_text = use_signal(|| node.text.clone());

    rsx! {
        li { class: "ml-4",
            div {
                class: "flex items-center gap-2 p-2 hover:bg-blue-900 rounded cursor-pointer",
                ondoubleclick: move |_| {
                    edit_modal_open.set(true);
                    edit_text.set(node.text.clone());
                },
                input {
                    r#type: "checkbox",
                    checked: node.completed,
                    onchange: move |_| {
                        completion_modal_open.set(true);
                    },
                }
                span {
                    class: format!(
                        "{}",
                        if node.completed {
                            "line-through text-gray-400"
                        } else {
                            match node.importance {
                                1 => "text-green-400",
                                2 => "text-red-400",
                                3 => "text-purple-400",
                                _ => "text-white",
                            }
                        },
                    ),
                    "{node.text}"
                }
                if let Some(deadline) = node.deadline {
                    div { class: "text-sm text-yellow-400 ml-2",
                        "{crate::business_logic::format_time_remaining(deadline)}"
                    }
                }
                if !node.children.is_empty() {
                    button {
                        class: "text-sm text-blue-500",
                        onclick: move |_| expanded.toggle(),
                        if expanded() {
                            "▼"
                        } else {
                            "▶"
                        }
                    }
                }
                button {
                    class: "text-sm text-green-500 ml-auto",
                    onclick: move |_| {
                        add_child(todos.write().as_mut(), node.id, next_id());
                        next_id += 1;
                    },
                    "+"
                }
                button {
                    class: "text-sm text-red-500",
                    onclick: move |_| {
                        delete_node(todos.write().as_mut(), node.id);
                    },
                    "✕"
                }
            }
            if expanded() && !node.children.is_empty() {
                ul { class: "ml-4 border-l-2 border-gray-300 pl-2",
                    for child in node.children.iter() {
                        TreeNode {
                            node: child.clone(),
                            todos,
                            next_id,
                            points,
                        }
                    }
                }
            }

            if edit_modal_open() {
                EditModal {
                    initial_text: node.text.clone(),
                    initial_importance: node.importance,
                    on_rename: move |(new_text, importance)| {
                        rename_node(todos.write().as_mut(), node.id, new_text, importance);
                        edit_modal_open.set(false);
                    },
                    on_cancel: move |_| {
                        edit_modal_open.set(false);
                    },
                }
            }
            if completion_modal_open() {
                CompletionModal {
                    task_name: node.text.clone(),
                    on_completed: move |_| {
                        let importance_points = calculate_importance_points(node.importance);
                        *points.write() += importance_points;
                        delete_node(todos.write().as_mut(), node.id);
                        completion_modal_open.set(false);
                    },
                    on_cancel: move |_| {
                        completion_modal_open.set(false);
                    },
                }
            }
        }
    }
}
