use levelcrush::{
    axum::{extract::State, routing::post, Json, Router},
    server::APIResponse,
    tracing,
    util::unix_timestamp,
};
use ts_rs::TS;

use crate::{
    app::state::AppState,
    database::{self, channel_log::ChannelLogRecord},
};

#[derive(serde::Serialize, serde::Deserialize, TS)]
#[ts(export, export_to = "../lib-levelcrush-ts/src/service-discord/")]
pub struct ChannelLogPayload {
    pub event_type: String,
    pub guild_id: String,
    pub category_id: String,
    pub channel_id: String,
    pub channel_name: String,
    pub message_id: String,
    pub message_timestamp: String,
    pub member_id: String,
    pub data: String,
}

pub fn router() -> Router<AppState> {
    Router::new().route("/log", post(create_log))
}

pub async fn create_log(
    State(state): State<AppState>,
    Json(payload): Json<ChannelLogPayload>,
) -> Json<APIResponse<bool>> {
    let mut response = APIResponse::new();

    tracing::info!("Logging message");

    let log = ChannelLogRecord {
        id: 0,
        guild_id: payload.guild_id.parse::<u64>().unwrap_or_default(),
        category_id: payload.guild_id.parse::<u64>().unwrap_or_default(),
        channel_id: payload.channel_id.parse::<u64>().unwrap_or_default(),
        channel_name: payload.channel_name,
        message_id: payload.message_id.parse::<u64>().unwrap_or_default(),
        message_timestamp: payload.message_timestamp.parse::<u64>().unwrap_or_default(),
        member_id: payload.member_id.parse::<u64>().unwrap_or_default(),
        data: payload.data,
        event_type: payload.event_type,
        created_at: unix_timestamp(),
        updated_at: 0,
        deleted_at: 0,
    };

    database::channel_log::create(log, &state.database).await;
    response.data(Some(true));

    response.complete();
    Json(response)
}
