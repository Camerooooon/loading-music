use dotenv::dotenv;
use std::{collections::HashMap, env, sync::Arc};

pub struct Data {
    sessions: Arc<Mutex<HashMap<GuildId, music::session::Session>>>,
    youtube: YouTube<HttpsConnector<HttpConnector>>,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

mod commands;
mod music;

use google_youtube3::{
    hyper::{self, client::HttpConnector},
    hyper_rustls::{self, HttpsConnector},
    oauth2, YouTube,
};
use poise::serenity_prelude::GuildId;
use serenity::prelude::GatewayIntents;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let discord_token = env::var("DISCORD_TOKEN")
        .expect("Missing DISCORD_TOKEN environment variable (fix this up)");
    let youtube_token = env::var("YOUTUBE_OAUTH_TOKEN")
        .expect("Missing YOUTUBE_OAUTH_TOKEN environment variable (fix this up)");
    let youtube_id = env::var("YOUTUBE_OAUTH_CLIENT_ID")
        .expect("Missing YOUTUBE_OAUTH_CLIENT_ID environment variable (fix this up)");

    let secret: oauth2::ApplicationSecret = oauth2::read_application_secret("secret.json")
        .await
        .expect("Could not find secret.json file");

    let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .build()
    .await
    .unwrap();
    let youtube = YouTube::new(
        hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .build(),
        ),
        auth,
    );

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::ping::ping(),
                commands::play::play(),
                commands::queue::queue(),
                commands::sessions::sessions(),
            ],
            ..Default::default()
        })
        .token(discord_token)
        .intents(GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    sessions: Arc::new(Mutex::new(HashMap::new())),
                    youtube,
                })
            })
        });

    println!("Client is starting wooo!");
    framework.run().await.unwrap();
}
