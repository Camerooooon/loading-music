use std::env;
use dotenv::dotenv;

pub struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

mod commands;

use serenity::prelude::GatewayIntents;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN environment variable (fix this up)");


    let framework = poise::Framework::builder()
    .options(poise::FrameworkOptions {
        commands: vec![commands::ping::ping()],
        ..Default::default()
    })
    .token(token)
    .intents(GatewayIntents::non_privileged())
    .setup(|ctx, _ready, framework| {
        Box::pin(async move {
            poise::builtins::register_globally(ctx, &framework.options().commands).await?;
            Ok(Data {})
        })
    });

    framework.run().await.unwrap();

    println!("Client is online");
}
