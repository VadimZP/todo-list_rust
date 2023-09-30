use chrono::prelude::*;
use std::io;

#[derive(Debug)]
struct Task {
    name: String,
    date: DateTime<Utc>,
    completed: bool,
}

impl Task {
    fn new(name: String, date: DateTime<Utc>, completed: bool) -> Self {
        if name.len() > 20 {
            panic!("Task name is too long");
        }

        Self {
            name,
            date,
            completed,
        }
    }

    fn complete(&mut self, value: String) {
        if self.name == value {
            self.completed = true;
        }
    }
}

struct TodoList {
    tasks: Vec<Task>,
}

enum Action {
    Create,
    Read,
    Complete,
    Delete,
    Unknown,
}

impl TodoList {
    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }
    fn complete_task(&mut self, index: usize, text: String) {
        self.tasks[index].complete(text);
        // self.tasks.remove(index);
    }
    fn read_task(&self, index: usize) -> Option<&Task> {
        self.tasks.get(index)
    }
}

fn action_handler(action: &str) -> (Action, Result<String, &str>) {
    match action {
        "create" => {
            println!("Type the name of your new task");

            let mut input = String::new();

            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let trimmed_string = input.trim().to_string();

                    return (Action::Create, Ok(trimmed_string));
                }
                Err(_) => (Action::Create, Err("Failed to save the input")),
            }
        }
        "read" => {
            println!("Type the number of the task you want to read");

            let mut input = String::new();

            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let trimmed_string = input.trim().to_string();

                    return (Action::Read, Ok(trimmed_string));
                }
                Err(_) => return (Action::Read, Err("Failed to read")),
            }
        }
        "complete" => {
            println!("Type the name of the task you want complete");

            let mut input = String::new();

            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let trimmed_string = input.trim().to_string();

                    return (Action::Complete, Ok(trimmed_string));
                }
                Err(_) => (Action::Complete, Err("Failed to complete the input")),
            }
        }
        _ => (Action::Unknown, Err("Unknown action")),
    }
}

fn main() {
    let mut todo_list = TodoList { tasks: Vec::new() };

    loop {
        println!("Type your action");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Err(error) => println!("Failed to save the input {error}"),
            Ok(_) => (),
        }

        let trimmed_string = input.trim().to_string().to_lowercase();

        let (action, payload) = action_handler(&trimmed_string);

        match payload {
            Ok(value) => match action {
                Action::Create => todo_list.add_task(Task::new(value, Utc::now(), false)),
                Action::Read => {
                    let index: usize = value.parse().expect("Failed to parse");
                    let task_of_index = todo_list.read_task(index);

                    match task_of_index {
                        Some(value) => println!("Task of index {index} - {:#?}", value),
                        None => println!("Task was not found"),
                    }
                }
                Action::Complete => {
                    let mut found_index: Option<usize> = None;

                    for (i, task) in todo_list.tasks.iter().enumerate() {
                        if task.name == value {
                            found_index = Some(i);
                        }
                    }

                    match found_index {
                        Some(index) => {
                            todo_list.tasks[index].complete(value);
                       
                            println!("{:#?}", todo_list.tasks);
                        }
                        None => println!("Task was not found"),
                    }
                }
                _ => println!("Failed to handle"),
            },
            Err(e) => println!("Payload error: {e}"),
        }
    }
}

// fn iterate_through_tasks(tasks: &Vec<String>) {
//     for (i, value) in tasks.iter().enumerate() {
//         println!("Task #{i} - {value}")
//     }
// }
