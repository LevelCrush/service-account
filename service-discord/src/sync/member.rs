use crate::routes::responses::DiscordUserResponse;
use levelcrush::{tokio, tracing, util::unix_timestamp, uuid::Uuid};
use sqlx::SqlitePool;

#[derive(Default, Clone, Debug)]
pub struct MemberSyncResult {
    pub display_name: String,
    pub username: String,
}

/// Syncs the api response from discord and returns a member sync result
pub async fn member(discord_user: DiscordUserResponse, pool: &SqlitePool) -> Option<MemberSyncResult> {
    None
}
