use crate::{
    app::{self, state::AppState},
    env::{self, AppVariable},
    sync,
};
use axum::Router;
use levelcrush::{
    axum::{
        self,
        extract::{Query, State},
        response::Redirect,
        routing::{get, post},
        Json,
    },
    axum_sessions::extractors::WritableSession,
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
        .route("/done", get(link_done))
        .route("/bad", get(link_bad))
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

async fn link_bungie(
    Query(query): Query<LinkQuery>,
    State(mut state): State<AppState>,
    mut session: WritableSession,
) -> Redirect {
    let member = {
        if let Some(code) = query.code {
            state.link_gens.access(&code).await
        } else {
            None
        }
    };

    if let Some(member) = member {
        app::session::login(&mut session, member);
        let done_url = format!("{}/link/done", env::get(AppVariable::HostAccounts));
        let redirect_url = format!(
            "{}/platform/bungie/login?redirect={}",
            env::get(AppVariable::HostAccounts),
            urlencoding::encode(&done_url)
        );
        Redirect::temporary(&redirect_url)
    } else {
        let redirect_url = format!("{}/link/bad", env::get(AppVariable::HostAccounts));
        Redirect::temporary(&redirect_url)
    }
}

async fn link_twitch() -> &'static str {
    "Hello twitch!"
}

async fn link_done() -> &'static str {
    "Thank you for linking your account. You can close this tab/window now"
}

async fn link_bad() -> &'static str {
    "Your code is either expired/incorrect or there was a problem starting the link brocess"
}
