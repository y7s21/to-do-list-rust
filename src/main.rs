use std::{fs, io::{self, Write}, string::String};
use serde::{Serialize, Deserialize};

const FILE_PATH: &str = "tasks.json";

#[derive(Serialize, Deserialize)]
struct Task {
    description: String,
    done: bool,
}

impl Task {
    fn new(description: &str) -> Task {
        Task {
            description: description.to_string(),
            done: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct TodoApp {
    tasks: Vec<Task>,
}

impl TodoApp {
    fn new() -> TodoApp {
        TodoApp { tasks: Vec::new() }
    }

    // Load tasks from file
    fn load_from_file() -> TodoApp {
        if let Ok(data) = fs::read_to_string(FILE_PATH) {
            if let Ok(todo_app) = serde_json::from_str::<TodoApp>(&data) {
                return todo_app;
            }
        }
        TodoApp::new()
    }

    // Save tasks to file
    fn save_to_file(&self) {
        let data = serde_json::to_string_pretty(&self).expect("Failed to serialize tasks");
        fs::write(FILE_PATH, data).expect("Failed to write tasks to file");
    }

    fn add_new_task(&mut self, description: &str) {
        let task = Task::new(description);
        self.tasks.push(task);
        self.save_to_file(); // Save to file after adding
    }

    fn mark_task_as_done(&mut self) {
        self.show_tasks();
        let index = match get_numeric_input("Enter the task index to mark as done: ") {
            Some(value) => value as usize,
            None => {
                println!("Invalid input, enter a valid number");
                return;
            }
        };

        if let Some(task) = self.tasks.get_mut(index - 1) {
            task.done = true;
            println!("Task '{}' marked as done.", task.description);
            self.save_to_file(); // Save to file after updating
        } else {
            println!("Invalid task index.");
        }

        println!("\nPress Enter to go back to the main menu...");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    }

    fn delete_task(&mut self) {
        self.show_tasks();
        let index = match get_numeric_input("Enter the task index to delete: ") {
            Some(value) => value as usize,
            None => {
                println!("Invalid input, enter a valid number");
                return;
            }
        };

        if index <= self.tasks.len() {
            let removed_task = self.tasks.remove(index - 1);
            println!("Task '{}' deleted.", removed_task.description);
            self.save_to_file(); // Save to file after deleting
        } else {
            println!("Invalid task index.");
        }

        println!("\nPress Enter to go back to the main menu...");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    }

    fn show_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks available.");
        } else {
            for (index, task) in self.tasks.iter().enumerate() {
                let status = if task.done { "[X]" } else { "[]" };
                println!("{}: {} {}", index + 1, status, task.description);
            }
        }
    }
}

fn main() {
    let mut todo_list_app = TodoApp::load_from_file();

    loop {
        println!("\n--- Todo List Menu ---");
        println!("1. Add new Task");
        println!("2. Mark Task As Done");
        println!("3. Show Tasks");
        println!("4. Delete Task");
        println!("5. Exit");

        let choice = match get_numeric_input("Enter your choice: ") {
            Some(value) => value,
            None => {
                println!("Invalid input, enter a valid number");
                continue;
            }
        };

        match choice {
            1 => {
                let description = get_string_input("Enter task description: ");
                todo_list_app.add_new_task(&description);
            }
            2 => todo_list_app.mark_task_as_done(),
            3 => {
                todo_list_app.show_tasks();
                println!("\nPress Enter to go back to the main menu...");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
            }
            4 => todo_list_app.delete_task(),
            5 => break,
            _ => println!("Invalid option, enter a number between 1-5"),
        }
    }
}

fn get_string_input(prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().ok();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read the input");
    input.trim().to_string()
}

fn get_numeric_input(prompt: &str) -> Option<u8> {
    print!("{}", prompt);
    std::io::stdout().flush().ok();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read the line");
    match input.trim().parse() {
        Ok(value) => Some(value),
        Err(_) => {
            println!("Invalid input, enter a valid number");
            None
        }
    }
}
