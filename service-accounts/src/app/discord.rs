use levelcrush::tracing;

use super::state::AppState;
use crate::{routes::responses::DiscordUserResponse, sync, sync::discord::MemberSyncResult};

/// queries a discord user directly by their discord id
pub async fn member_api(discord_id: &str, state: &AppState) -> Option<DiscordUserResponse> {
    let bot_token = std::env::var("DISCORD_BOT_TOKEN").unwrap_or_default();
    let bot_auth = format!("Bot {}", bot_token);
    let discord_user_id = discord_id;

    let endpoint = format!("https://discord.com/api/v10/users/{}", discord_user_id);
    let request = state
        .http_client
        .get(&endpoint)
        .header("Authorization", bot_auth)
        .send()
        .await;

    if let Ok(request) = request {
        let json = request.json::<DiscordUserResponse>().await;
        if let Ok(json) = json {
            Some(json)
        } else {
            let err = json.err().unwrap();
            tracing::error!("Error occurred while parsing user response:\r\n{}", err);
            None
        }
    } else {
        None
    }
}

/// fetches a discord member by their id and syncs the result
pub async fn member(discord_id: &str, state: &AppState) -> Option<MemberSyncResult> {
    let discord_response = member_api(discord_id, state).await;
    if let Some(user_response) = discord_response {
        sync::discord::member(user_response, &state.database).await
    } else {
        None
    }
}
