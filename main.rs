#[derive(Clone)] //derives attribute to implement clone
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

struct ToDoList {
    tasks: Vec<Task>,
}

impl ToDoList {
    fn new() -> ToDoList {
        ToDoList { tasks: Vec::new() }
    }

    fn add_task(&mut self, description: &str) -> Task {
        let id = self.tasks.len() + 1;
        let task = Task {
            id,
            description: String::from(description),
            completed: false,
        };
        self.tasks.push(task.clone());
        task
    }

    fn complete_task(&mut self, id: usize) -> Option<&Task> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            Some(task)
        } else {
            None
        }
    }

    // This function should print the details of all tasks in the ToDo list, including their ID, description, and completion status
    

    fn list_tasks(&self) {
        for task in &self.tasks {
            println!(
                "ID: {}, Description: {}, Completed: {}",
                task.id, task.description, task.completed
            );
        }
    }
}

fn main() {
    let mut todo_list = ToDoList::new();

    let task1 = todo_list.add_task("Complete Rust tutorial");
    let task2 = todo_list.add_task("Write a Rust program");
    let task3 = todo_list.add_task("Learn Rust ownership");

    todo_list.list_tasks();

    todo_list.complete_task(task2.id);

    println!("After completing task 2:");
    todo_list.list_tasks();
}
