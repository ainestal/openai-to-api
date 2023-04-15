mod chat_context;
mod chat_gpt;

use anyhow::Result;
use chat_gpt::ChatGPT;

#[tokio::main]
async fn main() -> Result<()> {
    // Getting the API key here
    let key = std::env::var("OPENAI_API_KEY")?;

    // Creating a new ChatGPT client.
    // Note that it requires an API key, and uses
    // tokens from your OpenAI API account balance.
    let mut gpt = ChatGPT::new(key)?;

    println!("Ready to chat!\n");

    loop {
        println!("You: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        println!("Response: ");
        let response = gpt.completion(input).await?;
        println!("{}", format!("{}", response.content));
    }
}
