use std::collections::HashMap;

use google_drive3::api::File;
/// Google Drive Interface for outputting destiny information
///
///
use google_drive3::{
    hyper::{self, client::HttpConnector},
    hyper_rustls::{self, HttpsConnector},
    oauth2, DriveHub,
};

use levelcrush::{anyhow, tracing};

#[derive(Debug, Clone)]
pub struct DriveDestinyPlayer {
    pub google_id: String,
    pub google_workbooks: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct DriveDestinyClans {
    pub google_id: String,
    pub google_workbooks: HashMap<String, String>,
    pub players: HashMap<String, DriveDestinyPlayer>,
}

#[derive(Debug, Clone)]
pub struct DriveDestinySeason {
    pub google_id: String,
    pub google_workbooks: HashMap<String, String>,
    pub clans: HashMap<String, DriveDestinyClans>,
}

#[derive(Clone)]
pub struct DriveDestinyReports {
    pub google_id: String,
    pub google_workbooks: HashMap<String, String>,
    pub seasons: HashMap<String, DriveDestinySeason>,
    pub clans: HashMap<String, DriveDestinyClans>,
    pub players: HashMap<String, DriveDestinyPlayer>,
    pub hub: DriveHub<HttpsConnector<HttpConnector>>,
}

#[derive(Debug, Clone)]
struct DriveFileList {
    pub folders: HashMap<String, String>,
    pub workbooks: HashMap<String, String>,
    pub nobucket: HashMap<String, google_drive3::api::File>,
}

/// iterates through a google sheets v3 file list and extracts files/folders as we specify
fn parse_filelist(filelist: &Vec<File>) -> DriveFileList {
    let mut drive_folders = HashMap::new();
    let mut drive_nobucket = HashMap::new();
    let mut drive_workbooks = HashMap::new();

    for file in filelist.iter() {
        // extract mime type
        let mime_type = match (&file.mime_type) {
            Some(data) => data.clone(),
            None => "".to_string(),
        };

        let resource_id = match (&file.id) {
            Some(data) => data.clone(),
            None => "".to_string(),
        };

        let name = match (&file.name) {
            Some(data) => data.clone(),
            None => "".to_string(),
        };

        // bucket appropriately
        match mime_type.as_str() {
            "application/vnd.google-apps.folder" => {
                drive_folders.insert(resource_id, name);
            }
            "application/vnd.google-apps.spreadsheet" => {
                drive_workbooks.insert(resource_id, name);
            }
            default => {
                drive_nobucket.insert(resource_id, file.clone());
            }
        }
    }

    DriveFileList {
        folders: drive_folders,
        workbooks: drive_workbooks,
        nobucket: drive_nobucket,
    }
}

impl DriveDestinyReports {
    /// Establishes a connection via  google drive api and handles authentication
    pub async fn get(drive_id: &str) -> anyhow::Result<DriveDestinyReports> {
        // construct hyper client
        tracing::info!("Constructing client");
        let hyper_client = hyper::Client::builder().build(
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
        let auth = oauth2::ServiceAccountAuthenticator::with_client(secret, hyper_client.clone())
            .build()
            .await?;
        let client = google_drive3::DriveHub::new(hyper_client, auth);

        Ok(DriveDestinyReports {
            google_id: drive_id.to_string(),
            google_workbooks: HashMap::new(),
            clans: HashMap::new(),
            players: HashMap::new(),
            seasons: HashMap::new(),
            hub: client,
        })
    }

    /// This will actually load in the information from the google drive
    pub async fn load(&mut self) -> anyhow::Result<()> {
        // clear hash maps
        self.clans.clear();
        self.players.clear();
        self.seasons.clear();

        let q_string = format!("'{}' in parents", self.google_id);

        /**
         * Folder Structure like so
         * Reports/Destiny (what our googledrive.reports.destiny variable points to)
         * |    Lifetime Overview Sheet: Google Sheet
         * |    [Season ...] : Folder
         * |    |  Season Overview Sheet : Google Sheet
         * |    | [Clan ...] : Folder
         * |    |   |   Clan Overview Sheet: Google Sheet
         * |    |   |   [Player ...] : Folder
         * |    |   |   |   Overview : Google sheet
         */
        // get the lifetime overview
        let (_, lifetime_filelist) = self.hub.files().list().q(&q_string).doit().await?;

        if let Some(lifetime_filelist) = lifetime_filelist.files {
            let parsed = parse_filelist(&lifetime_filelist);
            // debug
            println!("\r\n=======================\r\nFolders:\r\n{:?}", parsed.folders);
            println!("\r\n=======================\r\nWorkbooks:\r\n{:?}", parsed.workbooks);
            println!("\r\n=======================\r\nNot Bucketed:\r\n{:?}", parsed.nobucket);
        }

        Ok(())
    }
}
