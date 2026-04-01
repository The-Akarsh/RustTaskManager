use std::io::{ Write, stdin, stdout};
use colored::Colorize;

pub fn get_choice() -> u8 {
    println!();
    let menu: [&str; 5] = ["Add new task", "Edit task", "View task", "View all tasks", "Delete task" ];
    for (index,item) in menu.iter().enumerate(){
        println!(" {} {}", index+1, item);
    }
    let choice = read_u8("Enter your choice: ");
    choice
}

pub fn read_line (user_prompt : &str)  -> String{
    let mut input: String = String::new();

    print!("{}", user_prompt);
    std::io::stdout().flush().expect("Critical error: Could not flush stdout!");

    stdin().read_line(&mut input).expect("Critical error: Could not read input!");

   input.trim().to_string()
}

pub fn read_u8(user_prompt : &str) -> u8 {
    let mut input: String = String::new();
    loop {
        input.clear();
    
        print!("{}",user_prompt);
        stdout().flush().unwrap();
        
        stdin().read_line(&mut input).expect("Critical error: Failed to read from stdin");
        match input.trim().parse::<u8>() {
            Ok(num)  =>  return num,
            Err(_) =>{
                println!("{}","Error: Invalid input. Please enter a positive number!".red());
                0
            } 
        };

    }
}

// pub print_err()