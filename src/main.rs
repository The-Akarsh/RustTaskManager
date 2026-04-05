mod controllers;
mod models;
mod views;

use colored::Colorize;
use views::my_io::*;
use controllers::functions::*;
use  models::task_manager::TaskList;

fn main() {
    let mut task_list: TaskList = TaskList::new();
    println!("\t\tTask Manager");
    loop {
        println!();
        match get_choice() {
            1 => add_task(&mut task_list),
            2 => edit_task(&mut task_list),

            6 => {
                println!("{}","Exiting app!".green());
                break;
            }
            _=> println!("{}","Invalid option. Please try again!".red())
        }
    }
}
