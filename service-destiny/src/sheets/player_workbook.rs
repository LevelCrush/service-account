use google_sheets4::oauth2::{self};
use google_sheets4::{hyper, hyper_rustls, Sheets};
use google_sheets4::{hyper::client::HttpConnector, hyper_rustls::HttpsConnector};
use levelcrush::{anyhow, chrono, tokio, tracing};
use lib_destiny::app::report::member::MemberReport;
use lib_destiny::app::state::AppState;
use lib_destiny::env::Env;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PlayerWorkbookPlayer {
    pub bungie_name: String,
    pub bungie_id: String,
    pub last_activity_timestamp: String,
    pub top_activities: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum PlayerWorkbookActivityBreakdownType {
    DayOfWeek,
}

#[derive(Debug, Clone)]
pub struct PlayerWorkbookActivityBreakdown {
    pub breakdown_type: PlayerWorkbookActivityBreakdownType,
    pub data: HashMap<String, f64>,
}

#[derive(Debug, Clone)]
pub struct PlayerWorkbookActivity {
    pub instance_id: i64,
    pub name: String,
    pub completed: bool,
    pub timestamp: String,
}

#[derive(Clone)]
pub struct PlayerWorkbook {
    sheet_id: String,
    discord_id: String,
    discord_handle: String,
    bungie_name: String,
    bungie_id: i64,
    google: Sheets<HttpsConnector<HttpConnector>>,
    pub season: i32,
    pub last_updated: String,
    pub activity_lists: Vec<PlayerWorkbookActivity>,
    pub activity_breakdowns: Vec<PlayerWorkbookActivityBreakdown>,
    pub clan_member_list: HashMap<i64, PlayerWorkbookPlayer>,
    pub player_list: HashMap<i64, PlayerWorkbookPlayer>,
}

impl PlayerWorkbook {
    pub async fn connect(sheet_id: &str) -> anyhow::Result<PlayerWorkbook> {
        tracing::info!("Constructing client | Google Sheets");
        let client = hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_only()
                .enable_http1()
                .enable_http2()
                .build(),
        );

        tracing::info!("Constructing service key | Google Sheets");
        let secret = oauth2::read_service_account_key("google_credentials.json").await?;

        tracing::info!("Building authenticator | Google Sheets");
        let auth = oauth2::ServiceAccountAuthenticator::with_client(secret, client.clone())
            .build()
            .await?;

        tracing::info!("Constructing Sheets with client and auth");
        let google = Sheets::new(client, auth);

        tracing::info!("Done Constructing sheets interface");
        PlayerWorkbook::from_client(sheet_id, google)
    }

    pub fn from_client(
        sheet_id: &str,
        google: Sheets<HttpsConnector<HttpConnector>>,
    ) -> anyhow::Result<PlayerWorkbook> {
        Ok(PlayerWorkbook {
            sheet_id: sheet_id.to_string(),
            discord_id: "".to_string(),
            discord_handle: "".to_string(),
            bungie_name: "".to_string(),
            bungie_id: 0,
            season: 0,
            last_updated: "".to_string(),
            activity_lists: vec![],
            activity_breakdowns: vec![],
            clan_member_list: Default::default(),
            google,
            player_list: Default::default(),
        })
    }

    /// load data from the google sheet directly
    pub async fn load(&mut self) -> anyhow::Result<()> {
        //
        Ok(())
    }
}
