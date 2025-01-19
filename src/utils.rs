use crate::models::{AddTask, SupabaseClient, Task};
use colored::Colorize;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{self, ErrorKind};
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

pub async fn add_task(
    client: &SupabaseClient,
    title: String,
    description: Option<String>,
) -> Result<(), Box<dyn Error>> {
    println!(
        "{} :\n\t{}: {}\n\t{}: {}",
        "Adding task".bright_green(),
        "title".bright_yellow(),
        title.bright_cyan(),
        "description".bright_yellow(),
        description
            .clone()
            .unwrap_or(String::from(""))
            .bright_cyan()
    );

    let new_task = AddTask {
        title,
        description,
        is_completed: false,
    };

    let _ = client.post(new_task).await?;

    Ok(())
}

pub async fn list_tasks(client: &SupabaseClient) -> Result<(), Box<dyn Error>> {
    let response = client.get().await?;
    let tasks: Vec<Task> = response.json().await?;

    if tasks.is_empty() {
        println!("{}", "No tasks found.".bright_yellow());
        return Err(Box::new(io::Error::new(
            ErrorKind::NotFound,
            "No tasks found",
        )));
    }

    println!("{}", "Listing all tasks".bright_green());

    for task in tasks {
        let status = if task.is_completed {
            "✓".green()
        } else {
            "✗".red()
        };
        println!(
            "Task number: {}\n\t{}: {}\n\t{}: {}\n\t{}: {}",
            task.id.to_string().cyan(),
            "Status".bright_yellow(),
            status,
            "Title".bright_yellow(),
            task.title.bright_cyan(),
            "Description".bright_yellow(),
            task.description
                .unwrap_or(String::from("No Description Provided"))
                .bright_cyan(),
        );
    }

    Ok(())
}

pub async fn complete_task(client: &SupabaseClient, id: u32) -> Result<(), Box<dyn Error>> {
    // Send a PATCH request to update the row
    let value = serde_json::json!({ "is_completed": true });

    let response = client.patch(id, &value).await?;

    // Check if the request was successful
    if response.status().is_success() {
        let tasks = &response.json::<Vec<Task>>().await?;

        let updated_task = match tasks.first() {
            Some(task) => task,
            None => {
                println!("{}", "Task not found".bright_red());
                return Err(Box::new(io::Error::new(
                    ErrorKind::NotFound,
                    "Task not found",
                )));
            }
        };

        println!("{}", "Task completed successfully!".bright_green());

        let status = if updated_task.is_completed {
            "✓".green()
        } else {
            "✗".red()
        };

        println!(
            "Updated Task: {}\n\t{}: {}\n\t{}: {}\n\t{}: {}",
            updated_task.id.to_string().cyan(),
            "Status".bright_yellow(),
            status,
            "Title".bright_yellow(),
            updated_task.title.bright_cyan(),
            "Description".bright_yellow(),
            updated_task
                .description
                .clone()
                .unwrap_or(String::from("No Description Provided"))
                .bright_cyan(),
        );
    } else {
        println!("{}: {}", "Failed to update task".red(), response.status());
    }

    Ok(())
}

pub async fn delete_task(client: &SupabaseClient, id: u32) -> Result<(), Box<dyn Error>> {
    let response = client.delete(Some(id)).await?;

    if response.status().is_success() {
        let tasks = &response.json::<Vec<Task>>().await?;

        let deleted_task = match tasks.first() {
            Some(task) => task,
            None => {
                println!("{}", "Task not found".bright_red());
                return Err(Box::new(io::Error::new(
                    ErrorKind::NotFound,
                    "Task not found",
                )));
            }
        };

        println!("{}", "Task deleted successfully!".bright_green());

        let status = if deleted_task.is_completed {
            "✓".green()
        } else {
            "✗".red()
        };

        println!(
            "Deleted Task: {}\n\t{}: {}\n\t{}: {}\n\t{}: {}",
            deleted_task.id.to_string().cyan(),
            "Status".bright_yellow(),
            status,
            "Title".bright_yellow(),
            deleted_task.title.bright_cyan(),
            "Description".bright_yellow(),
            deleted_task
                .description
                .clone()
                .unwrap_or(String::from("No Description Provided"))
                .bright_cyan(),
        );
    } else {
        println!(
            "{}: {}",
            "Failed to delete task".bright_red(),
            response.status()
        );
    }

    Ok(())
}

pub async fn clear_tasks(client: &SupabaseClient) -> Result<(), Box<dyn Error>> {
    let response = client.delete(None).await?;

    // Check if the request was successful
    if response.status().is_success() {
        let tasks = &response.json::<Vec<Task>>().await?;
        if tasks.is_empty() {
            println!("{}", "No tasks with is_completed status found".bright_red());
            return Err(Box::new(io::Error::new(
                ErrorKind::NotFound,
                "No tasks with is_completed status found",
            )));
        } else {
            println!(
                "{} {}",
                tasks.len(),
                "completed tasks deleted successfully!".bright_green()
            );
        }
    } else {
        println!(
            "{}: {}",
            "Failed to delete tasks".bright_yellow(),
            format!("{}", response.status()).bright_red()
        );
    }

    Ok(())
}
