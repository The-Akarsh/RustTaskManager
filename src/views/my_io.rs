use std::io::{ Write, stdin, stdout};
use colored::Colorize;

/// Displays the main menu and returns the user's choice.
pub fn get_choice() -> u8 {
    println!();
    let menu: [&str; 6] = ["Add new task", "Edit task", "View task", "View all tasks", "Delete task","Exit" ];
    for (index,item) in menu.iter().enumerate(){
        println!(" {} {}", index+1, item);
    }
    let choice = read_u8("Enter your choice: ");
    choice
}

/// Reads a line of text from standard input.
///
/// Displays a prompt, flushes stdout to ensure immediate display, and reads
/// a complete line from the user. The input is trimmed and returned as a string.
///
/// # Arguments
///
/// * `user_prompt` - The message to display when prompting for input.
///
/// # Returns
///
/// The trimmed user input as a `String`, with leading/trailing whitespace removed.
///
/// # Panics
///
/// Panics if reading from stdin fails with a critical error.
pub fn read_line(user_prompt: &str) -> String {
    let mut input: String = String::new();

    print!("{}", user_prompt);
    stdout().flush().expect("Critical error: Could not flush stdout!");

    stdin().read_line(&mut input).expect("Critical error: Could not read input!");

    input.trim().to_string()
}

/// Reads and validates a positive 8-bit unsigned integer from the user.
///
/// Repeatedly prompts for input until a valid `u8` is provided. If invalid input
/// is received, displays an error message and retries. This is a blocking operation.
///
/// # Arguments
///
/// * `user_prompt` - The message to display when prompting for input.
///
/// # Returns
///
/// A valid `u8` value parsed from user input.
///
/// # Panics
///
/// Panics if reading from stdin fails with a critical error.
///
/// # Example
///
/// ```ignore
/// let choice = read_u8("Enter a number (0-255): ");
/// ```
pub fn read_u8(user_prompt: &str) -> u8 {
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
                continue;
            }
        };
    }
}

/// Reads a user input index and returns it if it is within the provided bounds. 
/// Loops until a valid index is provided.
/// 
/// # Arguments
/// * `user_prompt` - The message to display on the screen.
/// * `lower_limit` - The minimum valid input bound.
/// * `upper_limit` - The maximum valid input bound.
/// 
/// # Returns
/// Returns the zero-based index (`user_input - 1`).
pub fn read_valid_index(user_prompt: &str,lower_limit: usize, upper_limit: usize) -> usize {
    loop {
        let index = read_u8(user_prompt);
        let index: usize = index as usize;
    
        if index < lower_limit || index > upper_limit {
            println!("{}",format!("Invalid user input! The task number out of range. Must be between {} - {}!", lower_limit,upper_limit).red());
            continue;
        }

        return index - 1;
    }
}
// pub print_err()