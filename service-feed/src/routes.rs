use crate::AppState;
use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::http::{header, StatusCode};
use axum::routing::{get, post};
use axum::Router;
use levelcrush::cache::{CacheDuration, CacheValue};
use levelcrush::util::unix_timestamp;
use levelcrush::{axum, database, tracing};

pub fn router() -> Router<AppState> {
    Router::new().route("/:feed", get(read)).route("/:feed", post(write))
}

/// Reads from the target feed.
async fn read(headers: HeaderMap, Path(feed): Path<String>, State(mut state): State<AppState>) -> (HeaderMap, String) {
    // specify the set of headers that we want
    // we don't understand how to actually handle this feed since it's generic.
    // send it back down as plain text and let the application decide
    let mut output_headers = HeaderMap::new();
    output_headers.insert(header::CONTENT_TYPE, "text/plain".parse().unwrap());

    // extract the public key that should be in the header of our request
    let public_key = match headers.get("PUBLIC-KEY") {
        Some(header_value) => header_value.to_str().expect("Unable to convert header value to str"),
        _ => "",
    };

    // no public key? Don't even bother moving forward and don't waste querying the database
    if public_key.is_empty() {
        return (output_headers, String::new());
    }

    let cache_data = state.cache.access(&feed).await;
    let data = if cache_data.is_none() {
        // query database and obtain a record
        // when reading allow any valid access key to read from any feed
        let query = sqlx::query_file!("queries/feed_data_get.sql", public_key, feed)
            .fetch_optional(&state.database)
            .await;

        let data = if let Ok(query) = query {
            match query {
                Some(record) => Some(record.data),
                _ => None,
            }
        } else {
            database::log_error(query);
            None
        };

        let data = data.unwrap_or_default();

        state
            .cache
            .write(
                feed,
                CacheValue::with_duration(data.clone(), CacheDuration::HalfDay, CacheDuration::Day),
            )
            .await;

        data
    } else {
        cache_data.unwrap_or_default()
    };

    (output_headers, data)
}

/// Reads from the target feed.
async fn write(
    headers: HeaderMap,
    Path(feed): Path<String>,
    State(mut state): State<AppState>,
    body: String,
) -> (StatusCode, &'static str) {
    tracing::info!("Here attempting to write");
    // extract the public key that should be in the header of our request
    let public_key = match headers.get("PUBLIC-KEY") {
        Some(header_value) => header_value
            .to_str()
            .expect("Unable to convert public header value to str"),
        _ => "",
    };

    // extract private key that should be in the header of our request
    let private_key = match headers.get("PRIVATE-KEY") {
        Some(header_value) => header_value
            .to_str()
            .expect("Unable to convert private header value to str"),
        _ => "",
    };

    // we must have both public and private key to proceed.
    // don't even consider moving forward unless we have these
    if public_key.is_empty() || private_key.is_empty() {
        tracing::info!("No key provided");
        return (StatusCode::BAD_REQUEST, "No key provided");
    }

    let access_key_record = sqlx::query_file!("queries/access_key_get.sql", public_key, private_key)
        .fetch_one(&state.database)
        .await;

    // extract access key out of our record
    let access_key = match access_key_record {
        Ok(record) => record.id,
        _ => 0,
    };

    // no access key or an invalid record id returned , do not continue process
    if access_key <= 0 {
        tracing::info!("No key match");
        return (StatusCode::BAD_REQUEST, "Please provide a valid access key");
    }

    let feed_record = sqlx::query_file!("queries/feed_get.sql", access_key, feed)
        .fetch_optional(&state.database)
        .await;

    if let Ok(feed_record) = feed_record {
        let mut write_succeeded = false;
        if let Some(feed_record) = feed_record {
            tracing::info!("Updating feed target: {}", feed_record.id);
            // update the feed
            let result = sqlx::query_file!(
                "queries/feed_update.sql",
                body.as_str(),
                unix_timestamp(),
                feed_record.id
            )
            .execute(&state.database)
            .await;

            write_succeeded = result.is_ok();
            database::log_error(result);
        } else {
            tracing::info!("Creating feed target");
            // create the feed
            let result = sqlx::query_file!(
                "queries/feed_insert.sql",
                access_key,
                feed,
                body.as_str(),
                unix_timestamp()
            )
            .execute(&state.database)
            .await;

            write_succeeded = result.is_ok();
            database::log_error(result);
        }

        state
            .cache
            .write(
                feed,
                CacheValue::with_duration(body, CacheDuration::Persistant, CacheDuration::Persistant),
            )
            .await;

        if write_succeeded {
            (StatusCode::OK, "200 OK")
        } else {
            (StatusCode::NOT_MODIFIED, "304 Not Modified")
        }
    } else {
        database::log_error(feed_record);
        (StatusCode::NOT_MODIFIED, "304 Not Modified")
    }
}
