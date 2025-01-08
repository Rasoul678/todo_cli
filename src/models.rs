use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub is_completed: bool,
}

#[derive(Parser)]
#[command(name = "todo_cli")]
#[command(about = "A simple to-do list CLI application", long_about = None)]
#[command(version)]
pub struct Cli {
    /// Path to the tasks file (default: tasks.json)
    #[arg(short, long, value_name = "FILE")]
    pub file: Option<PathBuf>,

    /// Command to execute
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new task
    Add {
        /// Description of the task
        description: String,
    },
    /// List all tasks
    List,
    /// Mark a task as completed
    Complete {
        /// ID of the task to mark as completed
        id: u32,
    },
    /// Delete a task
    Delete {
        /// ID of the task to delete
        id: u32,
    },
    /// Clear all tasks
    Clear,
}
