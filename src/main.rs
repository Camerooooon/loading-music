use std::env;

use serenity::{prelude::GatewayIntents, Client};

#[tokio::main]
async fn main() {

    let token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN environment variable (fix this up)");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents).await.expect("Failed to create client");

    client.start().await.expect("Failed to start client");

    println!("Client is online");
}
