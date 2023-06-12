use crate::app::session::SessionKey;
use crate::database::platform::{AccountPlatformType, NewAccountPlatform};
use crate::database::platform_data::NewAccountPlatformData;
use crate::env::AppVariable;
use crate::routes::platform::{OAuthLoginQueries, OAuthLoginValidationQueries, OAuthLoginValidationRequest};
use axum::extract::{Query, State};
use axum::response::Redirect;
use axum::routing::get;
use axum::Router;
use axum_sessions::extractors::WritableSession;
use levelcrush::axum;
use levelcrush::axum_sessions;
use levelcrush::tracing;

use crate::app::state::AppState;
use crate::{app, database, env};
use levelcrush::util::unix_timestamp;

#[derive(serde::Serialize, serde::Deserialize, Default, Debug)]
pub struct DiscordValidationResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone)]
pub struct DiscordUserResponse {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub avatar: String,
    pub email: String,
    pub verified: bool,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", get(login))
        .route("/validate", get(validate))
}

pub async fn login(Query(login_fields): Query<OAuthLoginQueries>, mut session: WritableSession) -> Redirect {
    // make sure we know where to return our user to after they are done logging in
    let server_url = env::get(AppVariable::ServerUrl);
    let fallback_url = env::get(AppVariable::ServerFallbackUrl);
    let final_fallback_url = fallback_url;
    let final_redirect = login_fields.redirect.unwrap_or(final_fallback_url);

    let client_id = env::get(AppVariable::DiscordClientId);
    let authorize_redirect = env::get(AppVariable::DiscordValidateUrl);
    let scopes = vec!["identify", "email"].join("+");

    let hash_input = md5::compute(format!("{}||{}", client_id, unix_timestamp()));
    let discord_state = format!("{:x}", hash_input);
    let authorize_url = format!("https://discord.com/api/oauth2/authorize?response_type={}&client_id={}&scope={}&state={}&redirect_uri={}&prompt={}",
                                "code",
                                urlencoding::encode(client_id.as_str()),
                                scopes,
                                urlencoding::encode(discord_state.as_str()),
                                urlencoding::encode(authorize_redirect.as_str()),
                                "none"//"consent"
    );

    // store discord state check and final redirect in session
    app::session::write(SessionKey::PlatformDiscordState, discord_state, &mut session);

    // store original url that this route was called from
    app::session::write(SessionKey::PlatformDiscordCallerUrl, final_redirect, &mut session);

    // Now redirect
    Redirect::temporary(authorize_url.as_str())
}

pub async fn validate(
    Query(validation_query): Query<OAuthLoginValidationQueries>,
    State(state): State<AppState>,
    mut session: WritableSession,
) -> Redirect {
    let query_fields = validation_query;
    // make sure we know where to return our user to after they are done logging in
    let server_url = env::get(AppVariable::ServerUrl);
    let fallback_url = env::get(AppVariable::ServerFallbackUrl);
    let final_fallback_url = fallback_url;

    let final_redirect =
        app::session::read(SessionKey::PlatformDiscordCallerUrl, &session).unwrap_or(final_fallback_url);

    let mut do_process = true;
    let validation_state = query_fields.state.unwrap_or_default();
    let session_state = app::session::read::<String>(SessionKey::PlatformDiscordState, &session).unwrap_or_default();

    let oauth_code = query_fields.code.unwrap_or_default();
    let oauth_error = query_fields.error.unwrap_or_default();

    // make sure we don't have an error and we have a code that we can check
    if !oauth_error.is_empty() {
        do_process = false;
        tracing::warn!("There was an error found in the oauth request {}", oauth_error);
    }

    if oauth_code.is_empty() {
        tracing::warn!("There was no code present in the oauth request");
        do_process = false;
    }

    if validation_state != session_state {
        tracing::warn!(
            "Validation State and Session state did not match: Discord ({}) || Session({})",
            validation_state,
            session_state
        );
        do_process = false;
    }

    // if we are not yet allowed to process then go ahead and simply return immediately to our final redirect url that we know about
    if !do_process {
        return Redirect::temporary(final_redirect.as_str());
    }

    // now validate the code returned to us if we are allowed to process
    let mut validation_response = None;

    if do_process {
        let client_id = env::get(AppVariable::DiscordClientId);
        let client_secret = env::get(AppVariable::DiscordClientSecret);
        let authorize_redirect = env::get(AppVariable::DiscordValidateUrl);
        let scopes = vec!["identify", "email"].join("+");

        let request = state
            .http_client
            .post("https://discord.com/api/oauth2/token")
            .body(
                serde_urlencoded::to_string(OAuthLoginValidationRequest {
                    client_id: client_id.clone(),
                    client_secret: client_secret.clone(),
                    grant_type: "authorization_code".to_string(),
                    code: oauth_code.clone(),
                    redirect_uri: authorize_redirect.clone(),
                    scope: scopes,
                })
                .unwrap_or_default(),
            )
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Accept", "application/json")
            .send()
            .await;

        if request.is_ok() {
            let oauth_result = request.unwrap().json::<DiscordValidationResponse>().await;
            if oauth_result.is_ok() {
                validation_response = oauth_result.ok();
            } else {
                tracing::error!(
                    "Could not parse the oauth validation response for discord: \r\n{}",
                    oauth_result.err().unwrap()
                );
                validation_response = None;
            }
        }
    }

    do_process = validation_response.is_some();
    let mut discord_user = None;
    let mut access_token = String::new();
    let mut refresh_token = String::new();
    if do_process {
        let validation_response = validation_response.unwrap_or_default();
        access_token = validation_response.access_token.clone();
        refresh_token = validation_response.refresh_token.clone();

        let request = state
            .http_client
            .get("https://discord.com/api/v8/users/@me")
            .bearer_auth(access_token.clone())
            .send()
            .await;

        if request.is_ok() {
            let request = request.unwrap();
            let result = request.json::<DiscordUserResponse>().await;
            if result.is_ok() {
                discord_user = result.ok();
            } else {
                let error = result.err().unwrap();
                tracing::error!("{}", error);
                discord_user = None;
            }
        } else {
            let err = request.err().unwrap();
            tracing::error!("Could not parse the oauth validation response for discord:\r\n{}", err);
            discord_user = None;
        }
    }

    // now that we have information related to the discord user let's figure out if we can actually log
    do_process = discord_user.is_some();
    let mut account = None;
    let mut new_account = false;
    let discord_user = discord_user.unwrap_or_default();

    if do_process {
        tracing::info!("Attempting to match discord account account");
        account =
            database::platform::match_account(discord_user.id.clone(), AccountPlatformType::Discord, &state.database)
                .await;

        if account.is_none() {
            // no account found. Let's create an account first
            let timestamp = unix_timestamp();
            let token_seed = format!(
                "{}||{}||{}",
                timestamp,
                discord_user.id.clone(),
                discord_user.discriminator.clone()
            );
            let token_secret_seed = format!(
                "..{}..||..{}..||..{}..",
                token_seed.clone(),
                discord_user.email.clone(),
                timestamp
            );

            // create an account for this
            tracing::info!("Creating account");
            account = database::account::create(token_seed.clone(), token_secret_seed.clone(), &state.database).await;

            // only continue processing at this point if we have everything we need
            // for now, we allow processing if we have successfully
            do_process = account.is_some();
            new_account = true; // we created a new acc
        } else {
            // we do have an account that was pulled. So we can
            tracing::info!("Account already found");
            new_account = false;
            do_process = true;
        }
    }

    // only proceed if we are able to process
    let mut can_login = false;
    let account = account.unwrap_or_default();
    let mut account_platform = None;
    if do_process {
        if new_account {
            tracing::info!("New account setup and being linked");
            account_platform = database::platform::create(
                NewAccountPlatform {
                    account: account.id,
                    platform: AccountPlatformType::Discord,
                    platform_user: discord_user.id.clone(),
                },
                &state.database,
            )
            .await;

            // now that we have linked our account platform to our new account,
            // if we have done a successful database insert we can login
            can_login = account_platform.is_some();
        } else {
            tracing::info!("Account found and matched. Just login");

            // fetch the known account platform tied to this account
            account_platform =
                database::platform::from_account(&account, AccountPlatformType::Discord, &state.database).await;

            // we have an account that was matched we can just login
            can_login = account_platform.is_some();
        }
    }

    if can_login {
        // insert/update from our discord user response to update things like display name/etc
        let account_platform =
            account_platform.expect("No account platform was found. even though it should of been there");

        // everytime we log in, we are going to write out this information here
        let display_name = format!("{}#{}", discord_user.username, discord_user.discriminator);
        let data = vec![
            NewAccountPlatformData {
                key: "discord_id".to_string(),
                value: discord_user.id,
            },
            NewAccountPlatformData {
                key: "display_name".to_string(),
                value: display_name.clone(),
            },
            NewAccountPlatformData {
                key: "avatar".to_string(),
                value: discord_user.avatar,
            },
        ];

        // write the metadata out to be linked to the platform
        database::platform_data::write(&account_platform, &data, &state.database).await;

        // clear the session variables out, this is safe since discord is our primary login
        app::session::clear(&mut session);

        // in the session store important information related to the account, the account token and the token secret
        app::session::write(SessionKey::Account, account.token, &mut session);
        app::session::write(SessionKey::AccountSecret, account.token_secret, &mut session);
        app::session::write(SessionKey::DisplayName, display_name, &mut session);
    }

    // no matter what we redirect back to our caller
    Redirect::temporary(final_redirect.as_str())
}
