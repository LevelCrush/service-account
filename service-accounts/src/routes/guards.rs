use super::platform::OAuthLoginQueries;
use crate::app::session::SessionKey;
use crate::env::AppVariable;
use crate::{app, env};
use axum::extract::Query;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::{IntoResponse, Redirect, Response};
use axum_sessions::SessionHandle;
use levelcrush::axum::http::request::Parts;
use levelcrush::axum_sessions;
use levelcrush::{axum, tracing};
use urlencoding;

// checks to make sure their is a account session variable inside the user session
pub async fn session_logged_in<B>(req: Request<B>, next: Next<B>) -> Response {
    // read from session
    let mut account = String::new();
    if !req.extensions().is_empty() {
        let session_handle = req.extensions().get::<SessionHandle>().unwrap();
        let session: levelcrush::tokio::sync::RwLockReadGuard<'_, axum_sessions::async_session::Session> =
            session_handle.read().await;
        account = app::session::read::<String>(SessionKey::Account, &session).unwrap_or_default();
    }

    tracing::info!("{:?}", req.uri());
    if account.trim().is_empty() {
        let (request_parts, _) = req.into_parts();

        let req_query = match request_parts.uri.query() {
            Some(query) => format!("?{}", query.to_string()),
            _ => String::new(),
        };

        let redirect_url = format!("/login{}", req_query);
        Redirect::temporary(&redirect_url).into_response()
    } else {
        next.run(req).await
    }
}
