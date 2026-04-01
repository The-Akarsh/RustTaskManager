use crate::{models::{task::Task, task_manager::TaskList}, views::my_io::{read_line, read_u8}};

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