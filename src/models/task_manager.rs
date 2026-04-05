use std::future::pending;

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
    /** Returns the number of tasks in the list */
    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    /**  Returns true if the list has no tasks */
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    /** Checks if a specific funciion is pendion and return true if yes  */
    pub fn if_pending(&self,index: usize) -> bool{
         self.tasks[index].pending    
    }

    /** Prints task details of specific task  */
    pub fn view_task(&self,index: usize){
        println!("{}",self.tasks[index]);
    }

    pub fn view_all(&self){
        if self.is_empty() {
            println!("You have no tasks!");
            return;
        }

        println!("\n--- All Tasks ---");
        for (index, task) in self.tasks.iter().enumerate() {
            task.print_summary(index + 1);
        }
    }

    pub fn status_summary(&self) -> usize {
        self.tasks.iter().filter(|task| task.pending).count()
    }

    /** Add a new task to the list */
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    /** Replaces the speified task  with new task  */
    pub fn replace_task(&mut self,new_task:Task, index:usize){
        self.tasks[index] = new_task;
    }

    pub fn delete(&mut self, index: usize) -> String{
        let task =  self.tasks.remove(index);
        task.name
    }

    pub fn delete_all(&mut self){
        self.tasks.clear();
    }
}