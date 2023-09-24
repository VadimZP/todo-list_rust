use std::io;

struct TodoList {
    tasks: Vec<String>,
}

enum Action {
    Create,
    Read,
    Update,
    Delete,
    Error,
}

impl TodoList {
    fn tasks(&self) -> &Vec<String> {
        &self.tasks
    }
    fn add_task(&mut self, task: String) {
        self.tasks.push(task);
    }
    fn complete_task(&mut self, index: usize) {
        self.tasks.remove(index);
    }
    fn read_task(&self, index: usize) -> Option<&String> {
        self.tasks.get(index)
    }
}

fn action_handler(action: &str) -> (Action, String) {
    match action {
        "create" => {
            let mut input = String::new();

            match io::stdin().read_line(&mut input) {
                Ok(_) => println!("Input saved"),
                Err(error) => println!("Failed to save input {error}"),
            }

            let trimmed_string = input.trim().to_string();

            if trimmed_string.len() < 10 {
                return (Action::Create, trimmed_string);
            } else {
                return (Action::Error, "error".to_string());
            }
        }
        "read" => {
            let mut input = String::new();

            match io::stdin().read_line(&mut input) {
                Ok(_) => println!("Input saved"),
                Err(error) => return (Action::Error, "Failed to read".to_string()),
            }

            let trimmed_string = input.trim().to_string();

            return (Action::Read, trimmed_string);
        }
        _ => (Action::Error, "No such action".to_string()),
    }
}

fn main() {
    let mut todo_list = TodoList { tasks: Vec::new() };

    loop {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => println!("Input saved"),
            Err(error) => println!("Failed to save input {error}"),
        }

        let trimmed_string = input.trim().to_string();

        let (action, payload) = action_handler(&trimmed_string);

        match action {
            Action::Create => todo_list.add_task(payload),
            Action::Read => {
                let index: usize = payload.parse().expect("Failed to parse");
                let task_of_index = todo_list.read_task(index);

                match task_of_index {
                    Some(value) => println!("Task of index {index} - {value}"),
                    None => println!("Task was not found"),
                }
            }
            _ => println!("Failed to handle"),
        }

        println!("{:#?}", todo_list.tasks());
    }

    // todo_list.complete_task(0);
}

fn iterate_through_tasks(tasks: &Vec<String>) {
    for (i, value) in tasks.iter().enumerate() {
        println!("Task #{i} - {value}")
    }
}
