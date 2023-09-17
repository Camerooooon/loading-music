use std::{env, sync::Arc, collections::HashMap};
use dotenv::dotenv;

pub struct Data {
    sessions: Arc<Mutex<HashMap<GuildId, music::session::Session>>>
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

mod commands;
mod music;

use poise::serenity_prelude::GuildId;
use serenity::prelude::GatewayIntents;
use tokio::sync::Mutex;


#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN environment variable (fix this up)");


    let framework = poise::Framework::builder()
    .options(poise::FrameworkOptions {
        commands: vec![commands::ping::ping(), commands::play::play()],
        ..Default::default()
    })
    .token(token)
    .intents(GatewayIntents::non_privileged())
    .setup(|ctx, _ready, framework| {
        Box::pin(async move {
            poise::builtins::register_globally(ctx, &framework.options().commands).await?;
            Ok(Data {sessions: Arc::new(Mutex::new(HashMap::new()))})
        })
    });

    framework.run().await.unwrap();

    println!("Client is online");
}
