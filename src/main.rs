use std::path::PathBuf;

use lib::{
    add_task, clear_tasks, complete_task, delete_task, list_tasks, load_tasks, save_tasks, Cli,
    Colorize, Commands, Parser,
};

#[tokio::main]
async fn main() {
    // Parse command line arguments
    let cli = Cli::parse();

    // Determine the file path
    let file_path = cli.file.unwrap_or_else(|| PathBuf::from("tasks.json"));

    // Load tasks from the file
    let mut tasks = load_tasks(&file_path).await;

    match cli.command {
        Commands::Add { description } => {
            println!(
                "{} : {}",
                "Adding task".bright_green(),
                description.bright_cyan()
            );

            add_task(&mut tasks, description);
        }
        Commands::List => {
            println!("Listing all tasks");
            list_tasks(&tasks);
        }
        Commands::Complete { id } => {
            println!("Marking task {} as completed", id);
            complete_task(&mut tasks, id);
        }
        Commands::Delete { id } => {
            println!("Deleting task {}", id);
            delete_task(&mut tasks, id);
        }
        Commands::Clear => {
            println!("Clearing all tasks");
            clear_tasks(&mut tasks);
        }
    }

    // Save tasks to the file
    if let Err(e) = save_tasks(&tasks, &file_path).await {
        eprintln!("{}: {}", "Error saving tasks".red(), e);
    }
}
