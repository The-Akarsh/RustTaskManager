// Bring the formatting library into scope
use std::fmt;
use colored::Colorize; 

pub struct Task {
    pub name: String,
    pub description: String,
    pub category: String,
    pub importance: u8,
    pub pending: bool
}

// Impliments Task::new() function
impl Task {
    pub fn new(name: String, description: String, category: String, importance: u8) -> Task {
        Task{
            name,
            description,
            category,
            importance,
            pending : true
        }
    }

    pub fn print_summary(&self, id: usize) {
        let status = if self.pending { "Pending".yellow() } else { "Done".green() };
        println!(" [{}] {} ({})", id, self.name, status);
    }

    
}


// Implement the Display trait  our Task struct
impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status = if self.pending { "Pending".yellow() } else { "Done".red() };
        
        write!(
            f,
            "\n--- Task Details ---\nName:        {}\nDescription: {}\nCategory:    {}\nImportance:  {}\nStatus:      {}\n--------------------",
            self.name, self.description, self.category, self.importance, status
        )
    }
}