mod chat_context;
mod chat_gpt;
mod db_log;
mod frontal_lobe;
mod memory;

extern crate dotenv;

use anyhow::Result;
use chat_gpt::ChatGPT;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

use crate::{db_log::log_message, frontal_lobe::get_json_object};

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

    let mut brain_input = format!("You are connected to a database and you can store and retrieve memories.
    In order to do so, you need to provide me with the correct JSON format.
    For example, if you want to write a memory, you need to provide me with the following JSON:
    {{\"action\": \"read\", \"content\": \"ID of the memory\", \"comment\": \"Any text for the assistant to send messages\"}}
    {{\"action\": \"write\", \"content\": \"This is the content of the memory\", \"comment\": \"Any text for the assistant to send messages\"}}
    
    The actions you can do are:
    - read: read a memory
    - write: write a memory
    
    In order to read a memory you need to provide me with the ID of the memory. This will be returned when the memory is written.

    To be clear, you (the assistant) will be generating memories and asking me (the user) to store or read them from the database.
    
    Let's try, write a memory of something you like.");
    loop {
        // println!("You: ");
        // let mut input = String::new();
        // std::io::stdin().read_line(&mut input)?;
        println!("You: {}", brain_input.clone());

        // Store the message in the database
        log_message(
            "user".to_string(),
            brain_input.clone(),
            session_id.clone(),
            &pool,
        )
        .await?;

        let response = gpt.completion(brain_input.clone()).await?;
        println!("Response: ");
        println!("{}", format!("{}", response.content));

        brain_input = match get_json_object(response.content.clone()) {
            Ok(concept) => {
                // println!("Concept: {:?}", concept);
                match concept.action {
                    frontal_lobe::Action::Read => {
                        let memory =
                            memory::Memory::read_by_id(concept.content.unwrap(), &pool).await?;
                        format!("Memory retrieved. Content: {:?}", memory)
                    }
                    frontal_lobe::Action::Write => {
                        let memory = memory::Memory::new(
                            concept.content.unwrap(),
                            session_id.clone(),
                            &pool,
                        )
                        .await?;
                        format!("Memory saved. Memory ID: {}", memory.id)
                    }
                    frontal_lobe::Action::Archive => {
                        format!("Nothing done yet, I still can't archive memories")
                    }
                    frontal_lobe::Action::None => format!(
                        "I don't understand. You need to provide me with the correct JSON format"
                    ),
                }
            }
            Err(_) => {
                format!("I don't understand. You need to provide me with the correct JSON format")
            }
        };

        brain_input = format!(
            "Result: {brain_input}
Remember that you, assistant, need to ask me to store more memories or read them by providing me with the correct JSON format.
Do you want to read or write a memory?
"
        );

        // Log the message in the database
        log_message(response.role, response.content, session_id.clone(), &pool).await?;
    }
}
