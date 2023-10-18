use google_sheets4::api::{BatchClearValuesRequest, BatchUpdateValuesRequest, ValueRange};
use google_sheets4::hyper::Client;
use google_sheets4::oauth2::{self};
use google_sheets4::{hyper, hyper_rustls, Sheets};
use google_sheets4::{
    hyper::client::HttpConnector, hyper_rustls::HttpsConnector, oauth2::authenticator::Authenticator,
};
use levelcrush::{anyhow, project_str, tracing};
use lib_destiny::env::{AppVariable, Env};
use serenity::async_trait;
use serenity::framework::StandardFramework;
use serenity::model::prelude::{Guild, GuildId, Ready};
use serenity::prelude::{Context, EventHandler, GatewayIntents};

use crate::sheets::MasterWorkbook;

pub async fn sync(env: &Env) -> anyhow::Result<()> {
    tracing::info!("Constructing workbook connection");
    let sheet_id = env.get(AppVariable::MasterWorkSheet);
    let mut workbook = MasterWorkbook::get(&sheet_id).await?;

    tracing::info!("Loading information");
    workbook.load().await?;

    tracing::info!("Updating from API");
    workbook.api_sync(env).await?;

    tracing::info!("Saving workbook");
    workbook.save().await?;

    Ok(())
}

struct DiscordEventHandler;

#[async_trait]
impl EventHandler for DiscordEventHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Bot connected");
    }

    async fn cache_ready(&self, ctx: Context, guilds: Vec<GuildId>) {
        println!("Cache ready for use");

        for guild in guilds.into_iter() {
            let name = guild.name(&ctx);
            println!("{:?}", name);
        }
    }
}

pub async fn discord_sync(env: &Env) -> anyhow::Result<()> {
    tracing::info!("Constructing workbook connection");
    let sheet_id = env.get(AppVariable::MasterWorkSheet);
    let mut workbook = MasterWorkbook::get(&sheet_id).await?;

    tracing::info!("Loading workbook information");
    workbook.load().await?;

    // shadow into a non mutable state now that we have loaded
    let workbook = workbook;

    // construct discord bot to connect to any servers

    let discord_bot_token = env.get(AppVariable::DiscordBotToken);
    let discord_intents = GatewayIntents::all();
    let discord_client = serenity::Client::builder(&discord_bot_token, discord_intents)
        .event_handler(DiscordEventHandler)
        .await?;

    // loop throuhg clan sheets and match to player and pull get discord id if possible
    // construct tuple of (discord_id, clan_role)

    // loop throgugh vector of tuples and assign roles via bot

    // done!

    Ok(())
}

pub async fn discord_test(env: &Env) -> anyhow::Result<()> {
    let discord_bot_token = env.get(AppVariable::DiscordBotToken);
    let discord_intents = GatewayIntents::all();
    let discord_framework = StandardFramework::new();
    let mut discord_client = serenity::Client::builder(&discord_bot_token, discord_intents)
        .event_handler(DiscordEventHandler)
        .framework(discord_framework)
        .await?;

    if let Err(why) = discord_client.start().await {
        println!("Client Error: {:?}", why);
    }

    Ok(())
}
