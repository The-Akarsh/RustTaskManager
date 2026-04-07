mod controllers;
mod models;
mod views;

use colored::Colorize;
use controllers::functions::*;
use models::task_manager::TaskList;
use views::my_io::*;

fn main() {
    let mut task_list: TaskList = TaskList::load_from_file();
    println!();
    println!("\t\tTask Manager");
    loop {
        match get_choice() {
            1 => add_task(&mut task_list),
            2 => edit_task(&mut task_list),
            3 => view_task(&task_list),
            4 => task_list.view_all(),
            5 => delete_task(&mut task_list),
            6 => {
                println!();
                let file_path = format!("{}",task_list.save_to_file().display()).italic();
                println!("Succesfully saved tasks to {} ",file_path);
                print_green("Exiting app\t");
                break;
            }
            _ => print_err("Invalid option. Please try again!"),
        }
    }
}
