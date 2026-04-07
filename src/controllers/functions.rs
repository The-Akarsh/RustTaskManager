use crate::models::{task::Task, task_manager::TaskList};
use crate::views::my_io::*;

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
fn get_new_task() -> Task {
    let name: String = read_line("Name: ");
    let description: String = read_line("Description:\n");
    let category: String = read_line("Category: ");
    let importance: u8 = read_u8("Importance: ");

    Task::new(name, description, category, importance)
}



/// Prompts the user for task details and adds the new task to the `TaskList`.
/// 
/// # Arguments
/// * `task_list` - A mutable reference to the `TaskList`.
pub fn add_task(task_list: &mut TaskList) {
    println!("Creating new task...");
    let task: Task = get_new_task();
    task_list.add_task(task);
    task_list.save_to_file();
    print_green("Task added!");
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
    task_list.view_task(index);
    print_yellow("Leave fields as blank to use the previous value. Type '#' to leave the field as blank\n");

    if let Some(old_task) = task_list.get_task(index){
        let name = read_line_compare("Name: ", &old_task.name);
        let description = read_line_compare("Description: ", &old_task.description);
        let category = read_line_compare("Category: ", &old_task.category);
        let importance = read_u8_compare("Importance: ", &old_task.importance);
        
        let mut new_task = Task::new(name, description, category, importance);
        new_task.pending = !confirm_action("Mark as complete (y/n): ");

        task_list.replace_task(new_task, index);
        task_list.save_to_file();
        print_green("Task successfully updated!");
    }
    else {
        print_err("Error fetching data!");
    }
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
                task_list.save_to_file();
                let msg:String = format!("Deleted {}", deleted_name);
                print_err(&msg);
            } else {
                println!("Deletion cancelled!");
            }
        }
        2 => {
            let pending = task_list.status_summary();
            let prompt = format!("Are you sure you want to delete all tasks (Pending tasks: {})? (y/n): ", pending);
            
            if confirm_action(&prompt) {
                task_list.delete_all();
                task_list.save_to_file();
                print_err("Deleted all tasks!");
            } else {
                println!("Deletion cancelled!");
            }
        }
        _ => print_err("Invalid choice, returning to menu!"),
    }
}