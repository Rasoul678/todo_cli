use crate::models::Task;
use colored::Colorize;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::PathBuf;

pub async fn load_tasks(file_path: &PathBuf) -> Vec<Task> {
    // Try to open the file
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            println!(
                "{}: No tasks file found at {}. Starting with an empty list.",
                "Warning".yellow(),
                file_path.display()
            );
            return Vec::new(); // Return an empty vector if the file doesn't exist
        }
        Err(e) => {
            eprintln!(
                "{}: Failed to open tasks file at '{}': {}",
                "Error".red(),
                file_path.display(),
                e
            );
            return Vec::new(); // Return an empty vector if there's an error opening the file
        }
    };

    // Read the file contents into a string
    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        eprintln!(
            "{}: Failed to read tasks file at '{}': {}",
            "Error".red(),
            file_path.display(),
            e
        );
        return Vec::new(); // Return an empty vector if reading fails
    }

    // Deserialize the JSON string into a Vec<Task>
    match serde_json::from_str(&contents) {
        Ok(tasks) => tasks,
        Err(e) => {
            eprintln!(
                "{}: Invalid or corrupted tasks file at '{}': {}",
                "Error".red(),
                file_path.display(),
                e
            );
            Vec::new() // Return an empty vector if deserialization fails
        }
    }
}

pub async fn save_tasks(tasks: &Vec<Task>, file_path: &PathBuf) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;
    let json = serde_json::to_string(tasks)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn add_task(tasks: &mut Vec<Task>, description: String) {
    let new_task = Task {
        id: tasks.len() as u32 + 1,
        description,
        is_completed: false,
    };

    tasks.push(new_task);
}

pub fn list_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("{}", "No tasks found.".yellow());
        return;
    }

    for task in tasks {
        let status = if task.is_completed {
            "✓".green()
        } else {
            "✗".red()
        };
        println!(
            "{} {}: {}",
            status,
            task.id.to_string().cyan(),
            task.description
        );
    }
}

pub fn complete_task(tasks: &mut Vec<Task>, id: u32) {
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.is_completed = true;
        println!("{}: {}", "Task marked as completed".yellow(), id);
    } else {
        println!("{}: {}", "Task not found".red(), id);
    }
}

pub fn delete_task(tasks: &mut Vec<Task>, id: u32) {
    if let Some(index) = tasks.iter().position(|t| t.id == id) {
        tasks.remove(index);
        println!("{}: {}", "Task deleted".yellow(), id);
    } else {
        println!("{}: {}", "Task not found".red(), id);
    }
}

pub fn clear_tasks(tasks: &mut Vec<Task>) {
    tasks.clear();
    println!("{}", "All tasks cleared.".red().bold());
}
