use levelcrush::{database, util::unix_timestamp};
use sqlx::MySqlPool;

pub async fn deny(guild_id: u64, member_id: u64, role_name: &str, pool: &MySqlPool) {
    let queries = sqlx::query_file!(
        "queries/role_denies.sql",
        guild_id,
        member_id,
        role_name,
        unix_timestamp()
    )
    .execute(pool)
    .await;

    database::log_error(queries);
}

pub async fn allow(guild_id: u64, member_id: u64, role_name: &str, pool: &MySqlPool) {
    let queries = sqlx::query_file!("queries/role_allow.sql", guild_id, member_id, role_name)
        .execute(pool)
        .await;

    database::log_error(queries);
}
