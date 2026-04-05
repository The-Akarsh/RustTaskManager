use crate::models::{task::Task, task_manager::TaskList};
use crate::views::my_io::*;
use colored::Colorize;

/// Creates a new task by prompting the user for task details.
///
/// Requests the user to enter the task name, description, category, and importance level.
/// Returns a new `Task` instance with these details and pending status set to `true`.
///
/// # Returns
///
/// A newly created `Task` object.
///
/// # Note
///
/// This function includes a typo prompt ("Catagory") which should be fixed to "Category".
fn new_task() -> Task {
    let name: String = read_line("Name: ");
    let description: String = read_line("Description:\n");
    let category: String = read_line("Category: ");
    let importance: u8 = read_u8("Importance: ");

    Task::new(name, description, category, importance)
}

/// Prompts the user for a yes/no confirmation.
/// 
/// # Arguments
///
/// * `prompt` - The message to display to the user.
///
/// # Returns
///
/// `true` if the user confirms with "y" or "yes", `false` otherwise.
fn confirm_action(prompt: &str) -> bool {
    let response = read_line(prompt);
    match response.trim().to_lowercase().as_str() {
        "y" | "yes" => true,
        _ => false,
    }
}

/// Prompts the user for task details and adds the new task to the `TaskList`.
/// 
/// # Arguments
/// * `task_list` - A mutable reference to the `TaskList`.
pub fn add_task(task_list: &mut TaskList) {
    println!("Creating new task...");
    let task: Task = new_task();
    task_list.add_task(task);
}

/// Prompts the user to select and edit an existing task.
///
/// Displays all tasks, prompts the user to select a task by number, and replaces
/// the selected task with a new one created from user input. If the list is empty,
/// displays an appropriate message.
///
/// # Arguments
/// * `task_list` - A mutable reference to the `TaskList`.
pub fn edit_task (task_list: &mut TaskList) {

    if task_list.is_empty() {
        println!("You have no tasks to edit!");
        return;
    }
    let index = read_valid_index("Enter task number to edit: ", 1, task_list.len());
    task_list.replace_task(new_task(), index);
}

/// Prompts the user to select a task and displays its details.
/// 
/// # Arguments
/// * `task_list` - A reference to the `TaskList`.
pub fn view_task(task_list: &TaskList){
    if task_list.is_empty() {
        println!("You have no tasks to view!");
        return;
    }
    let index = read_valid_index("Enter task number: ", 1, task_list.len());
    task_list.view_task(index);
}

/// Prompts the user to delete a specific task or all tasks.
/// 
/// # Arguments
/// * `task_list` - A mutable reference to the `TaskList`.
pub fn delete_task(task_list: &mut TaskList){
    if task_list.is_empty() {
        println!("You have no tasks to delete!");
        return;
    }

    println!("\t1. Delete specific task\t2. Delete all tasks\n");

    match read_u8("Enter choice: ") {
        1 => {
            task_list.view_all();
            let index = read_valid_index("Enter task number: ", 1, task_list.len());
            let prompt = format!("Are you sure you want to delete task {}? (y/n): ", index + 1);
            if confirm_action(&prompt) {
                let deleted_name = task_list.delete(index);
                println!("{}", format!("Deleted {}", deleted_name).red());
            } else {
                println!("Deletion cancelled!");
            }
        }
        2 => {
            let pending = task_list.status_summary();
            let prompt = format!("Are you sure you want to delete all tasks (Pending tasks: {})? (y/n): ", pending);
            
            if confirm_action(&prompt) {
                task_list.delete_all();
                println!("{}", "Deleted all tasks!".red());
            } else {
                println!("Deletion cancelled!");
            }
        }
        _ => println!("{}", "Invalid choice, returning to menu!".red()),
    }
}