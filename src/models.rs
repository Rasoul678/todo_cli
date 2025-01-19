use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: Option<String>,
    pub is_completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddTask {
    pub title: String,
    pub description: Option<String>,
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
        /// Title of the task
        title: String,
        /// Description of the task
        description: Option<String>,
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

pub struct SupabaseClient {
    client: reqwest::Client,
    supabase_url: String,
    api_key: String,
}

impl SupabaseClient {
    pub fn new() -> Self {
        let supabase_url = env::var("SUPABASE_URL").expect("SUPABASE_URL not set");
        let api_key = env::var("SUPABASE_API_KEY").expect("SUPABASE_API_KEY not set");
        let client = reqwest::Client::new();

        Self {
            client,
            supabase_url,
            api_key,
        }
    }

    pub async fn post(&self, task: AddTask) -> reqwest::Result<reqwest::Response> {
        self.client
            .post(&format!("{}/rest/v1/tasks", self.supabase_url))
            .header("apikey", &self.api_key)
            .json(&task)
            .send()
            .await
    }

    pub async fn get(&self) -> reqwest::Result<reqwest::Response> {
        self.client
            .get(&format!("{}/rest/v1/tasks", self.supabase_url))
            .header("apikey", &self.api_key)
            .send()
            .await
    }

    pub async fn patch(&self, id: u32, value: &Value) -> reqwest::Result<reqwest::Response> {
        self.client
            .patch(&format!("{}/rest/v1/tasks?id=eq.{}", self.supabase_url, id))
            .header("apikey", &self.api_key)
            .header("Content-Type", "application/json")
            .header("Prefer", "return=representation")
            .json(&value)
            .send()
            .await
    }

    pub async fn delete(&self, id: Option<u32>) -> reqwest::Result<reqwest::Response> {
        let url = match id {
            Some(id) => format!("{}/rest/v1/tasks?id=eq.{}", self.supabase_url, id),
            None => format!("{}/rest/v1/tasks?is_completed=eq.true", self.supabase_url),
        };

        self.client
            .delete(&url)
            .header("apikey", &self.api_key)
            .header("Content-Type", "application/json")
            .header("Prefer", "return=representation")
            .send()
            .await
    }
}
