use levelcrush::{proc_macros::ExternalAPIResponse, tracing};
use lib_destiny::{
    app::state::AppState,
    env::{AppVariable, Env},
};

#[ExternalAPIResponse]
pub struct DiscordUserResponse {
    pub id: Option<String>,
    pub username: Option<String>,
    pub discriminator: Option<String>,
    pub avatar: Option<String>,
    pub global_name: Option<String>,
    pub display_name: Option<String>,
}

/// queries a discord user directly by their discord id
pub async fn member_api(discord_id: &str, env: &Env, state: &AppState) -> Option<DiscordUserResponse> {
    let bot_token = env.get(AppVariable::DiscordBotToken);
    let bot_auth = format!("Bot {}", bot_token);
    let discord_user_id = discord_id;

    let endpoint = format!("https://discord.com/api/v10/users/{}", discord_user_id);
    let request = state
        .bungie
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
