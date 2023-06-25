use levelcrush::{database, proc_macros::DatabaseResult, util::unix_timestamp};
use sqlx::MySqlPool;

#[DatabaseResult]
pub struct RoleDenyResult {
    pub member_id: u64,
}

pub async fn deny(guild_id: u64, member_id: u64, role_name: &str, pool: &MySqlPool) {
    let queries = sqlx::query_file!(
        "queries/role_denies.sql",
        guild_id,
        role_name,
        member_id,
        unix_timestamp()
    )
    .execute(pool)
    .await;

    database::log_error(queries);
}

pub async fn allow(guild_id: u64, member_id: u64, role_name: &str, pool: &MySqlPool) {
    let queries = sqlx::query_file!("queries/role_allow.sql", guild_id, role_name, member_id)
        .execute(pool)
        .await;

    database::log_error(queries);
}

pub async fn get_denies(guild_id: u64, role_name: &str, pool: &MySqlPool) -> Vec<RoleDenyResult> {
    let query = sqlx::query_file_as!(RoleDenyResult, "queries/role_denies_get.sql", guild_id, role_name)
        .fetch_all(pool)
        .await;

    if let Ok(query) = query {
        query
    } else {
        database::log_error(query);
        Vec::new()
    }
}
