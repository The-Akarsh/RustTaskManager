use crate::models::task::Task;
use std::fs;
use serde::{Deserialize, Serialize};
use directories::ProjectDirs;
use std::path::PathBuf;
/// A collection structure to hold and manage all tasks.
///
/// `TaskList` provides a container for multiple `Task` objects and exposes methods
/// for common operations like viewing, adding, editing, and deleting tasks.
/// It maintains an internal vector of tasks in insertion order.
#[derive(Serialize, Deserialize)]
pub struct TaskList {
    tasks: Vec<Task>,
}

impl TaskList {
    /// Creates a new, empty `TaskList`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let task_list = TaskList::new();
    /// assert!(task_list.is_empty());
    /// ```
    pub fn new() -> TaskList {
        TaskList {
            tasks: Vec::new(),
        }
    }

    /// Returns the number of tasks currently in the list.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut task_list = TaskList::new();
    /// assert_eq!(task_list.len(), 0);
    /// ```
    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    /// Checks whether the task list is empty.
    ///
    /// Returns `true` if there are no tasks in the list, `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    // Returns a reference to a specific task safely
    pub fn get_task(&self, index: usize) -> Option<&Task> {
        self.tasks.get(index)
    }

    /// Displays the full details of a specific task.
    ///
    /// # Arguments
    ///
    /// * `index` - The zero-based index of the task to display.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    pub fn view_task(&self, index: usize) {
        println!("{}", self.tasks[index]);
    }

    /// Displays a summary of all tasks in the list.
    ///
    /// If the list is empty, displays a message indicating there are no tasks.
    /// Pending tasks are shown in yellow, completed tasks in green.
    pub fn view_all(&self) {
        if self.is_empty() {
            println!("You have no tasks!");
            return;
        }

        println!("\n--- All Tasks ---");
        for (index, task) in self.tasks.iter().enumerate() {
            task.print_summary(index + 1);
        }
    }

    /// Returns the count of tasks that are not yet completed.
    ///
    /// # Returns
    ///
    /// The number of pending (incomplete) tasks in the list.
    pub fn status_summary(&self) -> usize {
        self.tasks.iter().filter(|task| task.pending).count()
    }

    /// Adds a new task to the end of the list.
    ///
    /// # Arguments
    ///
    /// * `task` - The `Task` to append to the list.
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    /// Replaces an existing task at the given index with a new task.
    ///
    /// # Arguments
    ///
    /// * `new_task` - The new `Task` to insert.
    /// * `index` - The zero-based index of the task to replace.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    pub fn replace_task(&mut self, new_task: Task, index: usize) {
        self.tasks[index] = new_task;
    }

    /// Removes and returns the name of the task at the given index.
    ///
    /// # Arguments
    ///
    /// * `index` - The zero-based index of the task to delete.
    ///
    /// # Returns
    ///
    /// The name of the deleted task.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    pub fn delete(&mut self, index: usize) -> String {
        let task = self.tasks.remove(index);
        task.name
    }

    /// Removes all tasks from the list.
    pub fn delete_all(&mut self) {
        self.tasks.clear();
    }


    pub fn get_file_path() -> PathBuf {
        const FILENAME: &str = "Tasks.json";
        // ProjectDirs::from(qualifier, organization, application_name)
        if let Some(proj_dirs) = ProjectDirs::from("com", "akarsh", "TaskManager") {
            let data_dir = proj_dirs.data_dir(); // Automatically finds AppData, Library, or .local
            
            fs::create_dir_all(data_dir).expect("Failed to create application data directory!");
            
            data_dir.join(FILENAME)
        } else {
            // Fallback: If the OS is completely unrecognizable, just save it locally
            PathBuf::from("tasks.json")
        }
    }

    pub fn save_to_file(&self) -> PathBuf {
        let json_string = serde_json::to_string_pretty(&self)
            .expect("Critical Error: Failed to serialize tasks.");

        let file_path = Self::get_file_path();

        fs::write(&file_path, json_string)
            .expect("Critical Error: Failed to write to hard drive.");
        file_path
    }

    pub fn load_from_file() -> TaskList {
        let file_path = Self::get_file_path();
        match fs::read_to_string(file_path) {
            Ok(json_string) => {
                serde_json::from_str(&json_string)
                    .expect("Critical Error: tasks.json is corrupted!")
            }
            Err(_) => {
                TaskList::new()
            }
        }
    }
}