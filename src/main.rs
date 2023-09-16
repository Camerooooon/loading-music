use std::env;
use dotenv::dotenv;

mod commands;

use serenity::{prelude::{GatewayIntents, EventHandler, Context}, Client, async_trait, model::prelude::{Interaction, InteractionResponseType, command::Command}};
use serenity::model::gateway::Ready;

struct Handler;

#[async_trait]
impl EventHandler for Handler {

    async fn interaction_create(&self, context: Context, interaction: Interaction) {

        if let Interaction::ApplicationCommand(command) = interaction {

            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                _ => "not implemented :(".to_string(),
            };

            if let Err(err) = command
                .create_interaction_response(&context.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", err);
            }
        }
        
    }

    async fn ready(&self, context: Context, ready: Ready) {

        println!("The bot is online! {}", ready.user.id);

        Command::create_global_application_command(context.http, |command| {
            commands::ping::register(command)
        }).await.expect("Could not generate ping command");



    }
    
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN environment variable (fix this up)");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents).event_handler(Handler).await.expect("Failed to create client");

    client.start().await.expect("Failed to start client");

    println!("Client is online");
}
