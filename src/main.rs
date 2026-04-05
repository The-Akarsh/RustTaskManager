mod controllers;
mod models;
mod views;

use colored::Colorize;
use controllers::functions::*;
use models::task_manager::TaskList;
use views::my_io::*;

fn main() {
    let mut task_list: TaskList = TaskList::load_from_file();
    println!("\t\tTask Manager");
    loop {
        println!();
        match get_choice() {
            1 => add_task(&mut task_list),
            2 => edit_task(&mut task_list),
            3 => view_task(&task_list),
            4 => task_list.view_all(),
            5 => delete_task(&mut task_list),
            6 => {
                let file_path = task_list.save_to_file();
                println!("Saving tasks to: {:?}",file_path );
                println!("{}", "Exiting app!".green());
                break;
            }
            _ => println!("{}", "Invalid option. Please try again!".red()),
        }
    }
}
