use dotenv::dotenv;

use lib::*;

#[tokio::main]
async fn main() {
    //! Load environment variables from .env file
    dotenv().ok();

    // Create a Supabase client
    let client = SupabaseClient::new();

    // Parse command line arguments
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { title, description } => {
            add_task(&client, title, description).await.ok();
        }
        Commands::List => {
            list_tasks(&client).await.ok();
        }
        Commands::Complete { id } => {
            complete_task(&client, id).await.ok();
        }
        Commands::Delete { id } => {
            delete_task(&client, id).await.ok();
        }
        Commands::Clear => {
            clear_tasks(&client).await.ok();
        }
    }
}
