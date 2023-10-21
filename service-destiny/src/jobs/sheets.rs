use std::collections::HashMap;
use std::process::exit;

use google_sheets4::api::{BatchClearValuesRequest, BatchUpdateValuesRequest, ValueRange};
use google_sheets4::hyper::Client;
use google_sheets4::oauth2::{self};
use google_sheets4::{hyper, hyper_rustls, Sheets};
use google_sheets4::{
    hyper::client::HttpConnector, hyper_rustls::HttpsConnector, oauth2::authenticator::Authenticator,
};
use levelcrush::{anyhow, project_str, tracing};
use lib_destiny::env::{AppVariable, Env};
use rand::distributions::Standard;
use serenity::async_trait;
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::StandardFramework;
use serenity::model::prelude::{Guild, GuildId, Ready};
use serenity::prelude::{Context, EventHandler, GatewayIntents};

use crate::sheets::{MasterWorkbook, WorksheetPlayer};

pub async fn sync(env: &Env) -> anyhow::Result<()> {
    tracing::info!("Constructing workbook connection");
    let sheet_id = env.get(AppVariable::MasterWorkSheet);
    let mut workbook = MasterWorkbook::get(&sheet_id).await?;

    tracing::info!("Loading information");
    workbook.load().await?;

    tracing::info!("Updating from API");
    workbook.api_sync(env).await?;

    tracing::info!("Generating reports");
    workbook.generate_reports(env).await?;

    tracing::info!("Saving workbook");
    workbook.save().await?;

    drop(workbook);

    tracing::info!("Syncing discord");
    discord_sync(env).await?;

    Ok(())
}

#[async_trait]
impl EventHandler for MasterWorkbook {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Bot connected");
    }

    async fn cache_ready(&self, ctx: Context, guilds: Vec<GuildId>) {
        println!("Cache ready for use");

        let mut clan_names = HashMap::new();
        for (clan_id, clan_data) in self.get_clans().iter() {
            clan_names.insert(clan_data.name.to_lowercase().trim().to_string(), *clan_id);
        }

        let workbook_players = self.get_players();
        for guild in guilds.into_iter() {
            let name = guild.name(&ctx).unwrap_or_default();
            let roles = guild.roles(&ctx).await.unwrap_or_default();

            // scan roles and find out which ones the server has that matches our tracking clans
            let mut clan_roles = HashMap::new();
            let mut clan_role_ids = Vec::new();
            for (role_id, role) in roles.iter() {
                let role = role.clone();
                let role_name = role.name.to_lowercase().trim().to_string();
                if let Some(clan_id) = clan_names.get(&role_name) {
                    clan_role_ids.push(role.id);
                    clan_roles.insert(clan_id, role);
                }
            }

            let mut members = guild.members(&ctx, None, None).await.unwrap_or_default();
            for member in members.iter_mut() {
                let display_name = member.display_name();
                let member_id = member.user.id.as_u64();
                tracing::info!("Scanning workbook for {}", member_id);
                // determine if this is a user we should even be worried about tracking
                let matching_players = workbook_players
                    .iter()
                    .filter_map(|(membership_id, player_data)| {
                        let discord_id = player_data.discord_id.parse::<u64>().unwrap_or_default();
                        if *member_id == discord_id {
                            Some(player_data.clone())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<WorksheetPlayer>>();

                // if it is, add it to our guild clan member array to track
                let first_match = matching_players.first();
                if let Some(matching_player) = first_match {
                    tracing::info!("Found matching player: {}", matching_player.discord_name);
                    // now find out which clan has this player
                    'clan_scan: for (clan_id, clan_data) in self.get_clans().iter() {
                        tracing::info!("Looking for {} in {}", matching_player.discord_name, clan_data.name);
                        for (bungie_membership_id, player_data) in clan_data.members.iter() {
                            let matching_player_bungie_id =
                                matching_player.bungie_membership_id.parse::<i64>().unwrap_or_default();

                            if matching_player_bungie_id == *bungie_membership_id {
                                tracing::info!(
                                    "Found clan  match for {} in {}",
                                    matching_player.discord_name,
                                    clan_data.name
                                );

                                // found the match for the player, now find the match for the clan role
                                if let Some(role) = clan_roles.get(clan_id) {
                                    let role_removal = member.remove_roles(&ctx, &clan_role_ids).await;
                                    if let Err(why) = role_removal {
                                        tracing::error!("Failed to prepare to remove roles:\r\n{:?}", why);
                                    }
                                    tracing::info!("Going to assign {} to {}", role.name, matching_player.discord_name);
                                    let add_role_result = member.add_role(&ctx, role.id).await;
                                    if let Err(why) = add_role_result {
                                        tracing::error!("Failed to add role\r\n{:?}", why);
                                    }
                                } else {
                                    tracing::warn!(
                                        "Unable to get matching clan role {} in {}\r\n{:?}",
                                        clan_data.name,
                                        name,
                                        clan_roles
                                    );
                                }
                                break 'clan_scan;
                            }
                        }
                    }
                }
            }
        }
        tracing::info!("Done assignment!\r\nCTRL+C to close");
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
    let mut discord_client = serenity::Client::builder(&discord_bot_token, discord_intents)
        .event_handler(workbook)
        .framework(StandardFramework::default())
        .await?;

    if let Err(why) = discord_client.start().await {
        tracing::error!("{:?}", why);
    }

    // loop throuhg clan sheets and match to player and pull get discord id if possible

    // construct tuple of (discord_id, clan_role)

    // loop throgugh vector of tuples and assign roles via bot

    // done!

    Ok(())
}
