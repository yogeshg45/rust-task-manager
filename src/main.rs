use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;

const FILE: &str = "tasks.json";

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    title: String,
    done: bool,
}

#[derive(Parser)]
#[command(name = "task", about = "A simple CLI task manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add { title: String },
    /// List all tasks
    List,
    /// Mark a task as done
    Done { id: usize },
    /// Delete a task
    Delete { id: usize },
}

fn load_tasks() -> Vec<Task> {
    match fs::read_to_string(FILE) {
        Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
        Err(_) => vec![],
    }
}

fn save_tasks(tasks: &Vec<Task>) {
    let data = serde_json::to_string_pretty(tasks).unwrap();
    fs::write(FILE, data).unwrap();
}

fn main() {
    let cli = Cli::parse();
    let mut tasks = load_tasks();

    match cli.command {
        Commands::Add { title } => {
            let id = tasks.len() + 1;
            tasks.push(Task { id, title: title.clone(), done: false });
            save_tasks(&tasks);
            println!("Added task {}: '{}'", id, title);
        }
        Commands::List => {
            if tasks.is_empty() {
                println!("No tasks yet. Add one with: task add \"your task\"");
            } else {
                for t in &tasks {
                    let status = if t.done { "✓" } else { "○" };
                    println!("[{}] {} - {}", status, t.id, t.title);
                }
            }
        }
        Commands::Done { id } => {
            match tasks.iter_mut().find(|t| t.id == id) {
                Some(task) => {
                    task.done = true;
                    save_tasks(&tasks);
                    println!("Task {} marked as done!", id);
                }
                None => println!("Task {} not found.", id),
            }
        }
        Commands::Delete { id } => {
            let before = tasks.len();
            tasks.retain(|t| t.id != id);
            if tasks.len() < before {
                save_tasks(&tasks);
                println!("Task {} deleted.", id);
            } else {
                println!("Task {} not found.", id);
            }
        }
    }
}