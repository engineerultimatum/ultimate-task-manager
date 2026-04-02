use crate::models::{TodoNode, TaskType};
use super::todo_factory::TodoNodeFactory;

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
            node.children.push(TodoNodeFactory::create_subtask(
                new_id,
                "New subtask".to_string(),
                1,
            ));
            return;
        }
        add_child_to_tree(&mut node.children, parent_id, new_id);
    }
}

/// Add a new root-level todo with specified type
pub fn add_root_node_with_type(todos: &mut Vec<TodoNode>, new_id: usize, task_type: TaskType) {
    let task = match task_type {
        TaskType::Regular => TodoNodeFactory::create_regular(new_id, "New task".to_string()),
        TaskType::Habit => TodoNodeFactory::create_habit(new_id, "New habit".to_string(), 1),
    };
    todos.push(task);
}

/// Delete a todo node and all its descendants from the tree
pub fn delete_node(nodes: &mut Vec<TodoNode>, id: usize) {
    nodes.retain(|node| node.id != id);
    for node in nodes {
        delete_node(&mut node.children, id);
    }
}

/// Reset habit completion status (called daily or when needed)
pub fn reset_habit_if_needed(nodes: &mut [TodoNode]) {
    use super::todo_factory::should_reset_habit;

    for node in nodes {
        if node.task_type == TaskType::Habit && should_reset_habit(node.last_completed_date) {
            node.completed = false;
            node.last_completed_date = None;
        }
        reset_habit_if_needed(&mut node.children);
    }
}

/// Mark a habit as completed without removing it
pub fn complete_habit(nodes: &mut [TodoNode], id: usize) {
    use super::todo_factory::get_today_timestamp;

    complete_habit_in_tree(nodes, id, get_today_timestamp());
}

fn complete_habit_in_tree(nodes: &mut [TodoNode], id: usize, today: u64) {
    for node in nodes {
        if node.id == id && node.task_type == TaskType::Habit {
            node.completed = true;
            node.last_completed_date = Some(today);
            return;
        }
        complete_habit_in_tree(&mut node.children, id, today);
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
