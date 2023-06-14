use levelcrush::database;
use levelcrush::util::unix_timestamp;
use levelcrush_macros::DatabaseRecord;
use sqlx::MySqlPool;
use std::collections::HashMap;

pub async fn exist<T: Into<String>, TS: Into<String>>(token: T, token_secret: TS, pool: &MySqlPool) -> bool {
    let query_result = sqlx::query!(
        r"
        SELECT COUNT(DISTINCT accounts.id) AS total
        FROM accounts
        WHERE accounts.token = ?
        AND token_secret = ?",
        token.into(),
        token_secret.into()
    )
    .fetch_one(pool)
    .await;
    if query_result.is_ok() {
        let record = query_result.unwrap();
        record.total > 0
    } else {
        false
    }
}
