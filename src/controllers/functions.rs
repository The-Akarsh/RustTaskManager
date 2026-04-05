use crate::models::{task::Task, task_manager::TaskList};
use crate::views::my_io::*;

/** Fn to read new task parameters */
fn new_task() -> Task {
    let name: String = read_line("Name: ");
    let description: String = read_line("Description:\n");
    let category: String = read_line("Catagory: ");
    let importance: u8 = read_u8("Importance: ");

    Task::new(name, description, category, importance)
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
