#[derive(Debug)]
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
}