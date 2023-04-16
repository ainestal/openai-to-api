mod chat_context;
mod chat_gpt;
mod db;

extern crate dotenv;

use anyhow::Result;
use chat_gpt::ChatGPT;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

use crate::db::store_message;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    // Creating a new session ID
    let session_id = Uuid::new_v4().to_string();

    // Init the environment variables
    let key = std::env::var("OPENAI_API_KEY")?;
    let db_url = std::env::var("DATABASE_URL")?;

    // Creating a new ChatGPT client.
    // Note that it requires an API key, and uses
    // tokens from your OpenAI API account balance.
    let mut gpt = ChatGPT::new(key, session_id.clone())?;

    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    println!("Ready to chat!\n");

    loop {
        println!("You: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        // Store the message in the database
        store_message("user".to_string(), input.clone(), session_id.clone(), &pool).await?;

        println!("Response: ");
        let response = gpt.completion(input).await?;
        println!("{}", format!("{}", response.content));

        // Store the message in the database
        store_message(response.role, response.content, session_id.clone(), &pool).await?;
    }
}
