use crate::{
    app::{self, state::AppState},
    sync,
};
use axum::Router;
use levelcrush::{
    axum::{
        self,
        extract::State,
        routing::{get, post},
        Json,
    },
    cache::{CacheDuration, CacheValue},
    server::APIResponse,
    tracing,
    util::unix_timestamp,
};

use super::responses::LinkGeneratedResponse;

#[derive(serde::Serialize, serde::Deserialize, Clone, Default, Debug)]
pub struct LinkGeneratePayload {
    pub id: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Default, Debug)]
pub struct LinkQuery {
    pub code: Option<String>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/generate", post(link_generate))
        .route("/bungie", get(link_bungie))
        .route("/twitch", get(link_twitch))
}

async fn link_generate(
    State(mut state): State<AppState>,
    Json(payload): Json<LinkGeneratePayload>,
) -> Json<APIResponse<LinkGeneratedResponse>> {
    let mut response = APIResponse::new();

    let member = app::discord::member(&payload.id, &state).await;
    if let Some(member) = member {
        let input = format!(
            "{}@{}::{}@{}",
            member.account_token_secret,
            member.username,
            unix_timestamp(),
            member.account_token,
        );
        let md5_digest = md5::compute(input);
        let hash = format!("{:x}", md5_digest);

        // store our hash
        // whena  user makes a request to /link/bungie or /link/twitch with  ?code=hash , if the has is found in link_gen cache, then we will trust them
        // this will only stay in the cache for 5 minutes.
        state
            .link_gens
            .write(
                hash.clone(),
                CacheValue::with_duration(member, CacheDuration::FiveMinutes, CacheDuration::FiveMinutes),
            )
            .await;

        response.data(Some(LinkGeneratedResponse { code: hash }));
    }

    response.complete();
    Json(response)
}

async fn link_bungie() -> &'static str {
    "Hello bungie!"
}

async fn link_twitch() -> &'static str {
    "Hello twitch!"
}
