use crate::{
    app::state::AppState,
    database,
    env::{self, AppVariable},
};
use levelcrush::axum::{
    self,
    extract::State,
    http::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use ts_rs::TS;

#[derive(serde::Serialize, serde::Deserialize, TS)]
#[ts(export, export_to = "../lib-levelcrush-ts/src/service-discord/")]
pub struct BotRoleSettingPayload {
    pub member_id: String,
    pub guild_id: String,
    pub role_name: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/bot/role/deny", post(role_deny))
        .route("/bot/role/allow", post(role_allow))
        .route_layer(axum::middleware::from_fn(can_write))
}

// checks to make sure their is a account session variable inside the user session
pub async fn can_write<B>(req: Request<B>, next: Next<B>) -> Response {
    let header_map = req.headers().clone();
    let access_key = match header_map.get("Access-Key") {
        Some(header_val) => header_val.to_str().unwrap_or_default(),
        _ => "",
    };

    let env_access = env::get(AppVariable::AccessKey);
    if access_key == env_access {
        next.run(req).await
    } else {
        (StatusCode::FORBIDDEN, "Not allowed").into_response()
    }
}

async fn role_deny(State(state): State<AppState>, Json(payload): Json<BotRoleSettingPayload>) -> &'static str {
    database::role::deny(
        payload.guild_id.parse::<u64>().unwrap_or_default(),
        payload.member_id.parse::<u64>().unwrap_or_default(),
        &payload.role_name,
        &state.database,
    )
    .await;
    "200 OK"
}

async fn role_allow(State(state): State<AppState>, Json(payload): Json<BotRoleSettingPayload>) -> &'static str {
    database::role::allow(
        payload.guild_id.parse::<u64>().unwrap_or_default(),
        payload.member_id.parse::<u64>().unwrap_or_default(),
        &payload.role_name,
        &state.database,
    )
    .await;
    "200 OK"
}
