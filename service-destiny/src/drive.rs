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

use crate::sheets::MasterWorkbook;
use levelcrush::anyhow::anyhow;
use levelcrush::tracing::instrument::WithSubscriber;
use levelcrush::{anyhow, tracing};

#[derive(Debug, Clone)]
pub struct DriveDestinyPlayer {
    pub google_id: String,
    pub google_workbooks: HashMap<String, String>,
    pub bungie_id: String,
    pub bungie_name: String,
}

#[derive(Debug, Clone)]
pub struct DriveDestinyClans {
    pub google_id: String,
    pub google_workbooks: HashMap<String, String>,
    pub name: String,
    pub group_id: i64,
    pub players: HashMap<String, DriveDestinyPlayer>,
}

#[derive(Debug, Clone)]
pub struct DriveDestinySeason {
    pub google_id: String,
    pub number: i32,
    pub google_workbooks: HashMap<String, String>,
    pub clans: HashMap<String, DriveDestinyClans>,
}

#[derive(Clone)]
pub struct DriveDestinyReports {
    pub google_id: String,

    /// Key = worksheet type, Value = google drive id
    pub google_workbooks: HashMap<String, String>,

    /// clans
    pub clans: HashMap<i64, String>,
    pub players: HashMap<i64, String>,

    /// Key = Google ID, Value = DriveDestinySeason
    pub seasons: HashMap<String, DriveDestinySeason>,
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

/// extract the workbooks associated at this file list level
fn extract_workbooks(parsed: &DriveFileList) -> HashMap<String, String> {
    let mut sheets = HashMap::new();
    for (google_id, name) in parsed.workbooks.iter() {
        let normalized_name = name.to_lowercase();
        let normalized_name = normalized_name.trim();
        let name_bits = normalized_name.split('-');

        // extract the last piece of text after the last '-' character
        // Lifetime - Overview = Overview
        // LifeTimeFooBar - Overview - ASDaojd = ASDaojd
        let worksheet_type = name_bits.last();
        if let Some(worksheet_type) = worksheet_type {
            let worksheet_Type = worksheet_type.to_lowercase();
            let worksheet_type = worksheet_type.trim();
            sheets.insert(worksheet_type.to_string(), google_id.clone());
        }
    }
    sheets
}

impl DriveDestinyReports {
    /// Establishes a connection via  google drive api and handles authentication
    pub async fn get(drive_id: &str) -> anyhow::Result<DriveDestinyReports> {
        // construct hyper client
        tracing::info!("Constructing client | Google Drive");
        let hyper_client = hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_only()
                .enable_http1()
                .enable_http2()
                .build(),
        );

        tracing::info!("Constructing service key | Google Drive");
        let secret = oauth2::read_service_account_key("google_credentials.json").await?;

        tracing::info!("Building authenticator | Google Drive");
        let auth = oauth2::ServiceAccountAuthenticator::with_client(secret, hyper_client.clone())
            .build()
            .await?;
        let client = google_drive3::DriveHub::new(hyper_client, auth);

        Ok(DriveDestinyReports {
            google_id: drive_id.to_string(),
            google_workbooks: HashMap::new(),
            seasons: HashMap::new(),
            players: HashMap::new(),
            clans: HashMap::new(),
            hub: client,
        })
    }

    /// loops through the seasons and gets the corrosponding drive id
    pub fn get_season(&self, number: i32) -> Option<String> {
        for (file_id, season) in self.seasons.iter() {
            if season.number == number {
                return Some(file_id.clone());
            }
        }
        None
    }

    pub fn get_season_clan(&self, season: i32, group_id: i64) -> Option<String> {
        for (file_id, drive_season) in self.seasons.iter() {
            if drive_season.number == season {
                for (clan_file_id, clan) in drive_season.clans.iter() {
                    if clan.group_id == group_id {
                        return Some(clan_file_id.clone());
                    }
                }
            }
        }
        None
    }

    pub fn get_season_clan_overview(&self, season: i32, group_id: i64) -> Option<String> {
        'drive_check: for (_, drive_season) in self.seasons.iter() {
            for (_, drive_clan) in drive_season.clans.iter() {
                if drive_clan.group_id == group_id {
                    if drive_clan.google_workbooks.contains_key("overview") {
                        return match drive_clan.google_workbooks.get("overview") {
                            Some(data) => Some(data.clone()),
                            None => None,
                        };
                    } else {
                        break 'drive_check;
                    }
                }
            }
        }
        None
    }

    /// Specifically create a new season folder in the drive
    pub async fn api_create_season(&mut self, number: i32) -> anyhow::Result<String> {
        let mut req = File::default();
        req.mime_type = Some("application/vnd.google-apps.folder".to_string());
        req.name = Some(format!("Season {}", number));
        req.parents = Some(vec![self.google_id.clone()]);

        let (_, file) = self
            .hub
            .files()
            .create(req)
            .upload(
                std::io::Cursor::new(""),
                "application/vnd.google-apps.folder".parse().unwrap(),
            )
            .await?;

        let google_id = file.id.unwrap_or_default();
        self.seasons.insert(
            google_id.clone(),
            DriveDestinySeason {
                google_id: google_id.clone(),
                number,
                google_workbooks: Default::default(),
                clans: Default::default(),
            },
        );

        Ok(google_id)
    }

    pub async fn api_create_season_clan(&mut self, season: i32, group_id: i64, name: String) -> anyhow::Result<String> {
        let season_google_id = match (self.get_season(season)) {
            Some(google_id) => google_id,
            None => self.api_create_season(season).await?,
        };

        let mut req = File::default();
        req.mime_type = Some("application/vnd.google-apps.folder".to_string());
        req.name = Some(format!("{} [{}]", name, group_id));
        req.parents = Some(vec![season_google_id.clone()]);

        let (_, file) = self
            .hub
            .files()
            .create(req)
            .upload(
                std::io::Cursor::new(""),
                "application/vnd.google-apps.folder".parse().unwrap(),
            )
            .await?;

        let google_id = file.id.unwrap_or_default();
        self.seasons.entry(season_google_id).and_modify(|season| {
            season.clans.insert(
                google_id.clone(),
                DriveDestinyClans {
                    google_id: google_id.clone(),
                    google_workbooks: Default::default(),
                    name: name.clone(),
                    group_id: group_id,
                    players: Default::default(),
                },
            );
            self.clans.insert(group_id, name);
        });
        Ok(google_id)
    }

    pub async fn api_create_season_clan_player(
        &mut self,
        season: i32,
        group_id: i64,
        clan_name: String,
        bungie_id: i64,
        bungie_name: &str,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn api_copy_overview(&mut self, season: i32, group_id: i64, clan_name: String) -> anyhow::Result<String> {
        // check what we have loaded in the drive and see if its possible for us to return an already existing result
        // if not, we will continue processing past this block
        tracing::warn!(
            " Attempting to copy from lifetime to Season {} | {} - Overview",
            season,
            clan_name
        );

        let folder_id = self.get_season_clan(season, group_id).unwrap_or_default();
        let worksheet = self.google_workbooks.get("overview").cloned();
        if let Some(worksheet) = worksheet {
            let new_name = format!("Season {} | {} - Overview", season, clan_name);
            let mut req = google_drive3::api::File::default();
            req.parents = Some(vec![folder_id]);
            req.name = Some(new_name.clone());
            let (_, data) = self.hub.files().copy(req, &worksheet).doit().await?;

            if let Some(workbook_google_id) = data.id {
                tracing::info!("Loading workbook to modify: {} ({})", new_name, workbook_google_id);
                let mut workbook = MasterWorkbook::connect(&workbook_google_id).await?;
                workbook.load().await?;

                tracing::info!("Trimming clans");
                let mut clan_keys = Vec::new();
                for (sheet_group_id, clan) in workbook.clans.iter() {
                    if group_id != *sheet_group_id {
                        clan_keys.push(*sheet_group_id);
                    }
                }

                // trim down to just the clan we want to demonstrate
                for group_id in clan_keys.into_iter() {
                    workbook.clans.remove(&group_id);
                }

                tracing::info!("Setting correct season from: {} to {}", workbook.season, season);
                workbook.season = season.to_string();

                tracing::warn!("Saving workbook: {}", new_name);
                workbook.save().await?;

                tracing::info!("Done saving workbook: {}", new_name);
                drop(workbook);

                Ok(workbook_google_id)
            } else {
                Err(anyhow!("Failed to copy workbook"))
            }
        } else {
            Err(anyhow!("No workbook to copy"))
        }
    }

    /// This will actually load in the information from the google drive
    pub async fn load(&mut self) -> anyhow::Result<()> {
        // clear hash maps
        self.seasons.clear();
        self.clans.clear();
        self.players.clear();

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
        let (season_folders, lifetime_workbooks) = if let Some(lifetime_filelist) = lifetime_filelist.files {
            let parsed = parse_filelist(&lifetime_filelist);
            // extract season folders
            let mut season_folders = HashMap::new();
            for (google_id, name) in parsed.folders.iter() {
                let normalized_name = name.to_lowercase();
                let normalized_name = normalized_name.trim();
                if normalized_name.starts_with("season") {
                    let mut name_bits = normalized_name.split(' ');
                    // Support formats like
                    // Season 22
                    // Season 22 - Electric Booga
                    // Season 22 | MySeasonNameHere r - Tomatoe
                    // Output for all of these should be 22
                    let season_number = name_bits.nth(1);
                    if let Some(season_number) = season_number {
                        let season_number = season_number.trim();
                        season_folders.insert(google_id.clone(), season_number.parse::<i32>().unwrap_or_default());
                    }
                }
            }

            // extract sheet types
            let sheets = extract_workbooks(&parsed);

            (season_folders, sheets)
        } else {
            (HashMap::new(), HashMap::new())
        };

        // store the lifetime workbooks
        self.google_workbooks = lifetime_workbooks;

        // now loop through each season folder
        if !season_folders.is_empty() {
            for (google_id, season_number) in season_folders.iter() {
                // track the season
                self.seasons.insert(
                    google_id.clone(),
                    DriveDestinySeason {
                        google_id: google_id.clone(),
                        google_workbooks: HashMap::new(),
                        clans: HashMap::new(),
                        number: *season_number,
                    },
                );
            }
        }

        // query each season folder and get all respective clan information
        for (season_google_id, season) in self.seasons.iter_mut() {
            let drive_q = format!("'{}' in parents", season_google_id);
            let (_, clan_file_list) = self.hub.files().list().q(&drive_q).doit().await?;

            if let Some(clan_file_list) = clan_file_list.files {
                let parsed = parse_filelist(&clan_file_list);

                // extract and store workbooks
                season.google_workbooks = extract_workbooks(&parsed);

                // create clan entry for the season
                for (google_id, name) in parsed.folders.iter() {
                    let mut clan_name = name.clone();
                    // support format
                    // Level Crush [groupidhere]
                    let components = clan_name.split(" ");
                    let (group_id, replace_subject) = if let Some(group_id_format) = components.last() {
                        let orig = group_id_format.to_string();
                        let mut chars = group_id_format.trim().chars();

                        // move forward one character
                        chars.next();

                        // ignore character at end
                        chars.next_back();

                        // output back
                        (chars.as_str().to_string(), orig)
                    } else {
                        ("".to_string(), "".to_string())
                    };

                    let trimmed_name = clan_name.replace(&replace_subject, "");
                    let trimmed_name = trimmed_name.trim();

                    season.clans.insert(
                        google_id.clone(),
                        DriveDestinyClans {
                            google_id: google_id.clone(),
                            google_workbooks: HashMap::new(),
                            name: trimmed_name.to_string(),
                            group_id: group_id.parse::<i64>().unwrap_or_default(),
                            players: HashMap::new(),
                        },
                    );

                    // track clan
                    let group_id = group_id.parse::<i64>().unwrap_or_default();
                    self.clans.entry(group_id).or_insert(trimmed_name.to_string());
                }
            }

            // now query each clan in that season folder
            for (google_clan_id, clan) in season.clans.iter_mut() {
                let drive_q = format!("'{}' in parents", clan.google_id);
                let (_, clan_file_list) = self.hub.files().list().q(&drive_q).doit().await?;
                if let Some(clan_file_list) = clan_file_list.files {
                    let parsed = parse_filelist(&clan_file_list);
                    clan.google_workbooks = extract_workbooks(&parsed);

                    for (google_player_id, name) in parsed.folders.iter() {
                        let name = name.clone();
                        let name = name.trim();

                        let components = name.split(" ");
                        let (bungie_id, replace_subject) = if let Some(bungie_id_format) = components.last() {
                            let orig = bungie_id_format.to_string();
                            let mut chars = bungie_id_format.trim().chars();

                            // move forward one character
                            chars.next();

                            // ignore character at end
                            chars.next_back();

                            // output back
                            (chars.as_str().to_string(), orig)
                        } else {
                            ("".to_string(), "".to_string())
                        };

                        let trimmed_name = name.replace(&replace_subject, "");
                        let trimmed_name = trimmed_name.trim();

                        clan.players.insert(
                            google_player_id.clone(),
                            DriveDestinyPlayer {
                                google_id: google_player_id.clone(),
                                google_workbooks: HashMap::new(),
                                bungie_id: bungie_id.clone(),
                                bungie_name: trimmed_name.to_string(),
                            },
                        );

                        // track player globally
                        let bungie_id = bungie_id.parse::<i64>().unwrap_or_default();
                        self.players.entry(bungie_id).or_insert(trimmed_name.to_string());
                    }

                    for (folder_id, player) in clan.players.iter_mut() {
                        let drive_q = format!("'{}' in parents", folder_id);
                        let (_, player_file_list) = self.hub.files().list().q(&drive_q).doit().await?;
                        if let Some(player_file_list) = player_file_list.files {
                            let parsed = parse_filelist(&player_file_list);
                            player.google_workbooks = extract_workbooks(&parsed);
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::env;
    use crate::sheets::MasterWorkbook;
    use levelcrush::tokio;
    use lib_destiny::env::AppVariable;
    use tracing_test::traced_test;

    #[tokio::test]
    #[traced_test]
    pub async fn testWorkbookChange() -> anyhow::Result<()> {
        let env = env::load();

        let mut workbook = MasterWorkbook::connect("1EMXGtaFyVnLSPHhUbA4VEd4A5xBGKW21StmGwR1PQx0").await?;
        workbook.load().await?;

        let target_clan = 5108335i64;
        workbook.clans.remove(&target_clan);

        let target_clan = 4250497i64;
        workbook.clans.remove(&target_clan);
        workbook.season = "23".to_string();

        /*
        tracing::info!("Sync");
        workbook.api_sync(&env).await?;

        tracing::info!("Generating reports");
        workbook.generate_reports(&env).await?; */

        tracing::info!("Saving");
        workbook.save().await?;

        drop(workbook);

        // remov eclan
        Ok(())
    }

    #[tokio::test]
    #[traced_test]
    pub async fn testDiveSearch() -> anyhow::Result<()> {
        let env = env::load();

        let master_drive_id = env.get(AppVariable::GoogleDriveReportDestinyFolder);
        let mut drive = DriveDestinyReports::get(&master_drive_id).await?;
        println!("Loading drive information");
        drive.load().await?;

        println!("Master Drive ID: {}", master_drive_id);

        // output all tracked season
        println!("================== Seasons ================");
        let mut counter = 0;
        for (drive_id, season) in drive.seasons.iter() {
            counter += 1;
            println!("{}. Season: {}", counter, season.number);
        }

        println!("================== Clans ================");
        let mut counter = 0;
        for (group_id, clan) in drive.clans.iter() {
            counter += 1;
            println!("{}. {} [{}]", counter, clan, group_id);
        }

        println!("================== Players ================");
        let mut counter = 0;
        for (player_id, player) in drive.players.iter() {
            counter += 1;
            println!("{}. {} [{}]", counter, player, player_id);
        }

        Ok(())
    }
}
