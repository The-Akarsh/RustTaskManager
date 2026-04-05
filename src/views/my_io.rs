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
pub fn read_valid_index(prompt: &str,lower_limit: usize, upper_limit: usize) -> usize {
    loop {
        let index = read_u8(prompt);
        let index: usize = index as usize;
    
        if index < lower_limit || index > upper_limit {
            println!("{}",format!("Invalid user input! The task number out of range. Must be between {} - {}!", lower_limit,upper_limit).red());
            continue;
        }

        return index - 1;
    }
}

/// Prompts the user for a yes/no confirmation.
/// 
/// # Arguments
///
/// * `prompt` - The message to display to the user.
///
/// # Returns
///
/// `true` if the user confirms with "y" or "yes", `false` otherwise.
pub fn confirm_action(prompt: &str) -> bool {
    let response = read_line(prompt);
    match response.trim().to_lowercase().as_str() {
        "y" | "yes" => true,
        _ => false,
    }
}

/// Reads a line of text, allowing the user to keep the old value or clear it.
///
/// Displays a prompt and reads user input. If the user simply presses Enter
/// (providing empty input), the `old_value` is returned. If the user enters
/// `#`, an empty string is returned (effectively clearing the field). Otherwise,
/// the new user input is returned.
///
/// # Arguments
///
/// * `prompt` - The message to display when prompting for input.
/// * `old_value` - The existing value to fall back on if input is empty.
///
/// # Returns
///
/// A `String` containing either the new input, the old value, or an empty string.
pub fn read_line_compare(prompt: &str, old_value: &str) -> String {

    let input = read_line(prompt);
    match input.as_str() {
        "" => old_value.to_string(),
        "#" => String::new(),
        _ => input
    }
}

/// Reads and validates an 8-bit unsigned integer, allowing the user to keep the old value or reset it.
///
/// Displays a prompt and reads user input. If the input is empty (Enter key),
/// the `old_value` is returned. If the input is `#`, it returns `0` (resetting the value).
/// Otherwise, it attempts to parse the input into a `u8`. If parsing fails,
/// it displays an error and prompts again.
///
/// # Arguments
///
/// * `prompt` - The message to display when prompting for input.
/// * `old_value` - A reference to the existing value to fall back on if input is empty.
///
/// # Returns
///
/// A valid `u8` representing the new value, the old value, or `0`.
pub fn read_u8_compare(prompt: &str, old_value: &u8) -> u8 {
    loop {
        let input = read_line(prompt);
        
        if input.is_empty() { return *old_value; }
        if input == "#" { return 0; } // Assuming # means reset importance to 0
        
        if let Ok(num) = input.parse::<u8>() {
            return num;
        }
        println!("{}", "Error: Invalid input! Please enter a number.".red());
    }
}
// pub print_err()