use crate::models::task::Task;

/**  A vector array to hold all tasks */
pub struct TaskList{
    tasks: Vec<Task>
}

impl TaskList {

    /** To create new TaskList array */
    pub fn new() -> TaskList {
        TaskList { 
            tasks: Vec::new()
         }
    }

    /** Add a new task to the list */
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }
}