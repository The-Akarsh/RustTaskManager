// Bring the formatting library into scope
use std::fmt;
use colored::Colorize;

/// Represents a single task with metadata and status tracking.
///
/// A `Task` encapsulates information about a to-do item, including its name,
/// description, category for organization, importance level, and completion status.
///
/// # Fields
///
/// * `name` - The title or name of the task.
/// * `description` - A detailed description of what the task entails.
/// * `category` - A classification category for organizing related tasks.
/// * `importance` - A priority level for the task (0-255).
/// * `pending` - Whether the task is incomplete (`true`) or completed (`false`).
pub struct Task {
    pub name: String,
    pub description: String,
    pub category: String,
    pub importance: u8,
    pub pending: bool,
}

impl Task {
    /// Creates a new task with the provided metadata.
    ///
    /// Initializes a new `Task` with all fields set to the provided values.
    /// The task is automatically set as pending (incomplete).
    ///
    /// # Arguments
    ///
    /// * `name` - The title of the task.
    /// * `description` - A detailed description of the task.
    /// * `category` - The category or classification for the task.
    /// * `importance` - The priority level of the task (0-255).
    ///
    /// # Returns
    ///
    /// A new `Task` instance with `pending` set to `true`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let task = Task::new(
    ///     "Buy groceries".to_string(),
    ///     "Get milk, bread, and eggs".to_string(),
    ///     "Shopping".to_string(),
    ///     5
    /// );
    /// ```
    pub fn new(name: String, description: String, category: String, importance: u8) -> Task {
        Task {
            name,
            description,
            category,
            importance,
            pending: true,
        }
    }

    /// Prints a concise summary of the task on a single line.
    ///
    /// Displays the task's ID, name, and completion status in color-coded format.
    /// Pending tasks are shown in yellow, while completed tasks are shown in green.
    ///
    /// # Arguments
    ///
    /// * `id` - The display ID (typically 1-based index) for the task.
    pub fn print_summary(&self, id: usize) {
        let status = if self.pending {
            "Pending".yellow()
        } else {
            "Done".green()
        };
        println!(" [{}] {} ({})", id, self.name, status);
    }
}


/// Implements custom formatting for displaying full task details.
impl fmt::Display for Task {
    /// Formats the task with all its details for display.
    ///
    /// Outputs a formatted block showing the complete task information including
    /// name, description, category, importance level, and completion status.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status = if self.pending {
            "Pending".yellow()
        } else {
            "Done".red()
        };

        write!(
            f,
            "\n--- Task Details ---\nName:        {}\nDescription: {}\nCategory:    {}\nImportance:  {}\nStatus:      {}\n--------------------",
            self.name, self.description, self.category, self.importance, status
        )
    }
}