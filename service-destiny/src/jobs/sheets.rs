use google_sheets4::hyper::Client;
use google_sheets4::oauth2::{self};
use google_sheets4::{hyper, hyper_rustls, Sheets};
use google_sheets4::{
    hyper::client::HttpConnector, hyper_rustls::HttpsConnector, oauth2::authenticator::Authenticator,
};
use levelcrush::{anyhow, project_str, tracing};
use std::collections::HashMap;

const GOOGLE_CREDENTIALS: &str = project_str!("google_credentials.json");

const SHEET_PLAYER_LIST: &str = "'Player List'";
const SHEET_TEMPLATE_ROSTER: &str = "'[Template] Roster'";

pub struct WorksheetPlayer {
    pub bungie_name: String,
    pub discord_name: String,
    pub discord_id: String,
}

pub struct WorksheetClan {
    pub name: String,
    pub group_id: i64,
    pub members: Vec<(String, u8)>,
}

pub struct MasterWorkbook {
    pub sheet_id: String,
    pub player_list: HashMap<String, WorksheetPlayer>,
    pub clans: Vec<WorksheetPlayer>,
    pub google: Sheets<HttpsConnector<HttpConnector>>,
}

impl MasterWorkbook {
    pub async fn get(sheet_id: &str) -> anyhow::Result<MasterWorkbook> {
        tracing::info!("Constructing client");
        let client = hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_only()
                .enable_http1()
                .enable_http2()
                .build(),
        );

        tracing::info!("Constructing service key");
        let secret = oauth2::read_service_account_key("google_credentials.json").await?;

        tracing::info!("Building authenticating");
        let auth = oauth2::ServiceAccountAuthenticator::with_client(secret, client.clone())
            .build()
            .await?;

        let google = Sheets::new(client.clone(), auth);
        let workbook = MasterWorkbook {
            sheet_id: sheet_id.to_string(),
            player_list: HashMap::new(),
            clans: Vec::new(),
            google,
        };

        Ok(workbook)
    }

    pub async fn hydrate(&mut self) -> anyhow::Result<()> {
        let (_, workbook) = self.google.spreadsheets().get(&self.sheet_id).doit().await?;

        let mut clan_sheets = Vec::new();
        let sheets = workbook.sheets.unwrap_or_default();
        for sheet in sheets.into_iter() {
            if let Some(properties) = sheet.properties {
                let sheet_title = properties.title.unwrap_or_default();
                if sheet_title.contains("[Clan]") {
                    clan_sheets.push(sheet_title);
                }
            }
        }

        tracing::info!("{:?}", clan_sheets);

        let player_sheet_range = format!("{SHEET_PLAYER_LIST}!A2:C");
        let (_, player_list_range) = self
            .google
            .spreadsheets()
            .get(&self.sheet_id)
            .add_ranges(&player_sheet_range)
            .include_grid_data(true)
            .doit()
            .await?;

        let sheets = player_list_range.sheets.unwrap_or_default();
        let player_sheet = sheets.first();
        if let Some(player_sheet) = player_sheet {
            let data = player_sheet.data.as_ref().expect("Expecting grid data");
            for grid_data in data.iter() {
                let row_data = grid_data.row_data.as_ref().expect("Expecting row data");
                for row in row_data.iter() {
                    if let Some(cell_data) = row.values.as_ref() {
                        tracing::info!("{:?}", cell_data);
                    } else {
                        tracing::info!("no data");
                    }
                }
            }
        }

        Ok(())
    }
}

pub async fn test_job() -> anyhow::Result<()> {
    tracing::info!("Constructing workbook connection");
    let mut workbook = MasterWorkbook::get("1vIfbDLZe7xf30pllymf5xPHABTLmMQezLAnChhzekoQ").await?;

    tracing::info!("Hydrating information");
    workbook.hydrate().await?;

    Ok(())
}
