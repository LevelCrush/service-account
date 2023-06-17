use levelcrush::tracing;

use super::state::AppState;
use crate::{
    env::{self, AppVariable},
    routes::responses::DiscordUserResponse,
    sync,
    sync::discord::MemberSyncResult,
};

/// queries a discord user directly by their discord id
pub async fn member_api(discord_id: &str, state: &AppState) -> Option<DiscordUserResponse> {
    let bot_token = env::get(AppVariable::DiscordBotToken);
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

/// query discord api with oauth authentication
pub async fn member_oauth_api(access_token: &str, state: &AppState) -> Option<DiscordUserResponse> {
    let request = state
        .http_client
        .get("https://discord.com/api/v10/users/@me")
        .bearer_auth(access_token.clone())
        .send()
        .await;

    if let Ok(result) = request {
        let json = result.json::<DiscordUserResponse>().await;
        if let Ok(data) = json {
            Some(data)
        } else {
            let err = json.err().unwrap();
            tracing::error!("Unable to parse oauth validation response: {}", err);
            None
        }
    } else {
        None
    }
}

/// Query a member via oauth authentication
/// Update them in our database
pub async fn member_oauth(access_token: &str, state: &AppState) -> Option<MemberSyncResult> {
    let oauth_response = member_oauth_api(access_token, state).await;
    if let Some(user_response) = oauth_response {
        sync::discord::member(user_response, &state.database).await
    } else {
        None
    }
}
