use crate::models::{task::Task, task_manager::TaskList};
use crate::views::my_io::*;
use colored::Colorize;

/** Fn to read new task parameters */
fn new_task() -> Task {
    let name: String = read_line("Name: ");
    let description: String = read_line("Description:\n");
    let category: String = read_line("Catagory: ");
    let importance: u8 = read_u8("Importance: ");

    Task::new(name, description, category, importance)
}

fn confirm_action(prompt: &str) -> bool {
    let response = read_line(prompt);
    match response.trim().to_lowercase().as_str() {
        "y" | "yes" => true,
        _ => false,
    }
}

pub fn add_task(task_list: &mut TaskList) {
    println!("Creating new task...");

    let task: Task = new_task();
    task_list.add_task(task);
}

pub fn edit_task (task_list: &mut TaskList) {

    let index = read_valid_index("Enter task number to edit: ", 1, task_list.len());
    task_list.replace_task(new_task(), index);

}

pub fn view_task(task_list: &TaskList){
    let index = read_valid_index("Enter task number: ", 0, task_list.len());
    task_list.view_task(index);
}


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