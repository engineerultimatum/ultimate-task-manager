use crate::models::TodoNode;

/// Rename an existing todo node
pub fn rename_node(todos: &mut Vec<TodoNode>, id: usize, new_text: String, importance: u32) {
    rename_node_in_tree(todos, id, &new_text, importance);
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

/// Add a child todo node to a parent
pub fn add_child(nodes: &mut Vec<TodoNode>, parent_id: usize, new_id: usize) {
    add_child_to_tree(nodes, parent_id, new_id);
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
                deadline: None,
            });
            return;
        }
        add_child_to_tree(&mut node.children, parent_id, new_id);
    }
}

/// Add a new root-level todo
pub fn add_root_node(todos: &mut Vec<TodoNode>, new_id: usize) {
    todos.push(TodoNode {
        id: new_id,
        text: "New task".to_string(),
        completed: false,
        importance: 1,
        children: vec![],
        deadline: None,
    });
}

/// Delete a todo node and all its descendants from the tree
pub fn delete_node(nodes: &mut Vec<TodoNode>, id: usize) {
    nodes.retain(|node| node.id != id);
    for node in nodes {
        delete_node(&mut node.children, id);
    }
}

/// Calculate points awarded for completing a task based on importance
pub fn calculate_importance_points(importance: u32) -> u32 {
    match importance {
        1 => 1,
        2 => 2,
        3 => 3,
        _ => 1,
    }
}
