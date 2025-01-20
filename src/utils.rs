use crate::models::{AddTask, SupabaseClient, Task};
use colored::Colorize;
use std::error::Error;
use std::io::{self, ErrorKind};

pub async fn add_task(
    client: &SupabaseClient,
    title: String,
    description: Option<String>,
) -> Result<(), Box<dyn Error>> {
    println!("{}", "Adding task...".bright_green(),);

    let new_task = AddTask {
        title,
        description,
        is_completed: false,
    };

    let response = client.post(new_task).await?;

    // Check if the request was successful
    if response.status().is_success() {
        let tasks = &response.json::<Vec<Task>>().await?;

        let added_task = match tasks.first() {
            Some(task) => task,
            None => {
                println!("{}", "Task not added".bright_red());
                return Err(Box::new(io::Error::new(
                    ErrorKind::NotFound,
                    "Task not added",
                )));
            }
        };
        print_success_message(added_task, "Task added successfully!");
    } else {
        println!(
            "{}: {}",
            "Failed to add task".bright_red(),
            response.status()
        );
    }

    Ok(())
}

pub async fn list_tasks(
    client: &SupabaseClient,
    is_completed: Option<bool>,
) -> Result<(), Box<dyn Error>> {
    println!("{}", "Listing all tasks...".bright_green());

    let response = client.get(is_completed).await?;
    let tasks: Vec<Task> = response.json().await?;

    if tasks.is_empty() {
        println!("{}", "No tasks found.".bright_yellow());
        return Err(Box::new(io::Error::new(
            ErrorKind::NotFound,
            "No tasks found",
        )));
    }

    for task in tasks {
        print_success_message(&task, "---".repeat(20).as_str());
    }

    Ok(())
}

pub async fn complete_task(client: &SupabaseClient, id: u32) -> Result<(), Box<dyn Error>> {
    println!("{}", "Completing task...".bright_green());

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

        print_success_message(&updated_task, "Task completed successfully!");
    } else {
        println!("{}: {}", "Failed to update task".red(), response.status());
    }

    Ok(())
}

pub async fn delete_task(client: &SupabaseClient, id: u32) -> Result<(), Box<dyn Error>> {
    println!("{}", "Deleting task...".bright_green());

    // Send a DELETE request to delete the row
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

        print_success_message(&deleted_task, "Task deleted successfully!");
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
    println!("{}", "Clearing all tasks...".bright_green());

    // Send a DELETE request to delete all rows
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

fn print_success_message(task: &Task, message: &str) {
    println!("{}", message.bright_green(),);

    let status = if task.is_completed {
        "✓".bright_green().bold()
    } else {
        "✗".bright_red().bold()
    };

    println!(
        "Task:\n  {}: {}\n  {}: {}\n  {}: {}\n  {}: {}",
        "id".bright_yellow(),
        task.id.to_string().bright_cyan(),
        "Status".bright_yellow(),
        status,
        "Title".bright_yellow(),
        task.title.bright_cyan(),
        "Description".bright_yellow(),
        task.description
            .clone()
            .unwrap_or(String::from("No Description Provided"))
            .bright_cyan(),
    );
}
