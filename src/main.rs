use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[derive(Clone, PartialEq)]
struct TodoNode {
    id: usize,
    text: String,
    completed: bool,
    importance: u32, // 1 = green, 2 = red, 3 = purple
    children: Vec<TodoNode>,
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Main {}
    }
}

#[component]
pub fn Main() -> Element {
    let mut todos = use_signal(|| vec![
        TodoNode {
            id: 0,
            text: "Learn Rust".to_string(),
            completed: false,
            importance: 1,
            children: vec![
                TodoNode { id: 1, text: "New Subtask".to_string(), completed: false, importance: 1, children: vec![] },
                TodoNode { id: 2, text: "New Subtask".to_string(), completed: false, importance: 1, children: vec![] },
            ],
        },
    ]);
    let mut next_id = use_signal(|| 3);
    let points = use_signal(|| 0u32);

    rsx! {
        div { class: "container mx-auto p-8",
            div { class: "flex justify-between items-center mb-6",
                h1 { class: "text-3xl font-bold", "UTiM" }
                div { class: "text-2xl font-bold text-yellow-400", "Points: {points}" }
            }
            button {
                class: "text-lg text-green-500 mb-4 px-4 py-2 bg-green-700 rounded hover:bg-green-600",
                onclick: move |_| {
                    add_root_node(&mut todos, next_id());
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

#[component]
fn TreeNode(node: TodoNode, todos: Signal<Vec<TodoNode>>, mut next_id: Signal<usize>, mut points: Signal<u32>) -> Element {
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
                        add_child(&mut todos, node.id, next_id());
                        next_id += 1;
                    },
                    "+"
                }
                button {
                    class: "text-sm text-red-500",
                    onclick: move |_| {
                        delete_node(&mut todos, node.id);
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
                        rename_node(&mut todos, node.id, new_text, importance);
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
                        let importance_points = match node.importance {
                            1 => 1,
                            2 => 2,
                            3 => 3,
                            _ => 1,
                        };
                        *points.write() += importance_points;
                        delete_node(&mut todos, node.id);
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

#[component]
fn EditModal(
    initial_text: String,
    initial_importance: u32,
    on_rename: EventHandler<(String, u32)>,
    on_cancel: EventHandler<()>,
) -> Element {
    let mut input_value = use_signal(|| initial_text.clone());
    let mut importance = use_signal(|| initial_importance);

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
fn CompletionModal(
    
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

fn update_node_completed(todos: &mut Signal<Vec<TodoNode>>, id: usize, completed: bool) {
    todos.with_mut(|t| {
        update_node_in_tree(t, id, completed);
    });
}

fn update_node_in_tree(nodes: &mut [TodoNode], id: usize, completed: bool) {
    for node in nodes {
        if node.id == id {
            node.completed = completed;
            return;
        }
        update_node_in_tree(&mut node.children, id, completed);
    }
}

fn rename_node(todos: &mut Signal<Vec<TodoNode>>, id: usize, new_text: String, importance: u32) {
    todos.with_mut(|t| {
        rename_node_in_tree(t, id, &new_text, importance);
    });
}

fn rename_node_in_tree(nodes: &mut [TodoNode], id: usize, new_text: &str, importance: u32) {
    for node in nodes {
        if node.id == id {
            node.text = new_text.to_string();
            node.importance = importance;
            return;
        }
        rename_node_in_tree(&mut node.children, id, new_text, importance);
    }
}

fn add_child(todos: &mut Signal<Vec<TodoNode>>, parent_id: usize, new_id: usize) {
    todos.with_mut(|t| {
        add_child_to_tree(t, parent_id, new_id);
    });
}

fn add_child_to_tree(nodes: &mut [TodoNode], parent_id: usize, new_id: usize) {
    for node in nodes {
        if node.id == parent_id {
            node.children.push(TodoNode {
                id: new_id,
                text: "New subtask".to_string(),
                completed: false,
                importance: 1,
                children: vec![],
            });
            return;
        }
        add_child_to_tree(&mut node.children, parent_id, new_id);
    }
}

fn delete_node(todos: &mut Signal<Vec<TodoNode>>, id: usize) {
    todos.with_mut(|t| {
        delete_node_from_tree(t, id);
    });
}

fn delete_node_from_tree(nodes: &mut Vec<TodoNode>, id: usize) {
    nodes.retain(|node| node.id != id);
    for node in nodes {
        delete_node_from_tree(&mut node.children, id);
    }
}

fn add_root_node(todos: &mut Signal<Vec<TodoNode>>, new_id: usize) {
    todos.with_mut(|t| {
        t.push(TodoNode {
            id: new_id,
            text: "New task".to_string(),
            completed: false,
            importance: 1,
            children: vec![],
        });
    });
}
