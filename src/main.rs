mod controllers;
mod models;
mod views;

use views::my_io::*;
use crate::{controllers::functions::add_task, models::task_manager::TaskList};

fn main() {
    let mut task_list: TaskList = TaskList::new();
    println!("\t\tTask Manager");
    loop {
        println!();
        match get_choice() {
            1 => add_task(&mut task_list),

            _=> println!()
        }
    }
}
