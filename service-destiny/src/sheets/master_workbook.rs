use std::collections::HashMap;
use std::time::Duration;

use google_sheets4::api::{BatchClearValuesRequest, BatchUpdateValuesRequest, ValueRange};
use google_sheets4::oauth2::{self};
use google_sheets4::{hyper, hyper_rustls, Sheets};
use google_sheets4::{hyper::client::HttpConnector, hyper_rustls::HttpsConnector};
use serde_json::Value;

use levelcrush::chrono;
use levelcrush::{anyhow, tracing};
use lib_destiny::app::report::member::MemberReport;
use lib_destiny::app::state::AppState;
use lib_destiny::env::Env;

use crate::discord;

const SHEET_PLAYER_LIST: &str = "'Player List'";
const SHEET_TEMPLATE_ROSTER: &str = "'[Template] Roster'";

#[derive(Debug, Clone)]
pub struct WorksheetPlayer {
    pub bungie_name: String,
    pub discord_name: String,
    pub discord_id: String,
    pub bungie_membership_id: String,
    pub bungie_platform: String,
}

#[derive(Debug, Clone)]
pub struct WorksheetClanMember {
    pub name: String,
    pub bungie_id: i64,
    pub role: i64,
    pub last_online: String,
    pub seasonal_activities: i64,
    pub seasonal_activities_with_clan: i64,
    pub seasonal_activities_with_clan_percent: f64,
    pub frequent_clan_members: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct WorksheetClan {
    pub name: String,
    pub group_id: i64,
    pub members: HashMap<i64, WorksheetClanMember>,
    pub members_sorted: Vec<i64>,
}

#[derive(Clone)]
pub struct MasterWorkbook {
    sheet_id: String,
    season: String,
    last_updated: String,
    player_list: HashMap<String, WorksheetPlayer>,
    clans: HashMap<i64, WorksheetClan>,
    google: Sheets<HttpsConnector<HttpConnector>>,
    player_reports: HashMap<String, MemberReport>,
    player_list_sorted: Vec<String>,
}

impl MasterWorkbook {
    /// construct a workbook connection
    pub async fn connect(sheet_id: &str) -> anyhow::Result<MasterWorkbook> {
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

        let google = Sheets::new(client, auth);
        Ok(MasterWorkbook::new(sheet_id, google))
    }

    /// Directly construct a Master Workbook based off the supplying sheet id and google sheet client
    pub fn new(sheet_id: &str, google: Sheets<HttpsConnector<HttpConnector>>) -> MasterWorkbook {
        MasterWorkbook {
            sheet_id: sheet_id.to_string(),
            player_list: HashMap::new(),
            last_updated: "".to_string(),
            clans: HashMap::new(),
            player_reports: HashMap::new(),
            player_list_sorted: Vec::new(),
            season: "".to_string(),
            google,
        }
    }

    /// get players
    pub fn get_players(&self) -> &HashMap<String, WorksheetPlayer> {
        &self.player_list
    }

    pub fn get_clans(&self) -> &HashMap<i64, WorksheetClan> {
        &self.clans
    }

    pub fn get_sheet_id(&self) -> &str {
        &self.sheet_id
    }

    pub fn get_season(&self) -> &str {
        &self.season
    }

    pub fn get_lastupdated(&self) -> &str {
        &self.last_updated
    }

    /// this will populate our Masterworkbook data structure with data from the spreadsheet
    /// this will make 0 api calls to the bungie api
    /// use this function to provide a state that is READ FROM THE SPREADSHEET
    pub async fn load(&mut self) -> anyhow::Result<()> {
        // clear arrays
        self.clans.clear();
        self.player_list.clear();

        let base_string = String::new();

        let (_, workbook) = self.google.spreadsheets().get(&self.sheet_id).doit().await?;

        // grab all clan sheet names ahead of time
        let mut clan_sheet_names = Vec::new();
        let sheets = workbook.sheets.unwrap_or_default();
        for sheet in sheets.into_iter() {
            if let Some(properties) = sheet.properties {
                let sheet_title = properties.title.unwrap_or_default();
                if sheet_title.starts_with("[Clan]") {
                    clan_sheet_names.push(sheet_title);
                }
            }
        }

        // info sheet parsing
        let info_sheet_range = format!("'Info'!B1:B2");
        let (_, info_range) = self
            .google
            .spreadsheets()
            .get(&self.sheet_id)
            .add_ranges(&info_sheet_range)
            .include_grid_data(true)
            .doit()
            .await?;

        let sheets = info_range.sheets.unwrap_or_default();
        let info_sheet = sheets.first();
        if let Some(info_sheet) = info_sheet {
            if let Some(data) = info_sheet.data.as_ref() {
                for grid_data in data.iter() {
                    if let Some(row_data) = grid_data.row_data.as_ref() {
                        for row in row_data.iter() {
                            let mut txt_values = Vec::new();
                            if let Some(cell_data) = row.values.as_ref() {
                                txt_values.extend(
                                    cell_data
                                        .iter()
                                        .map(|v| v.formatted_value.as_ref().unwrap_or(&base_string).clone())
                                        .collect::<Vec<String>>(),
                                );
                            }

                            // this works for now, but it is very limited to this particular scenario of usage
                            // for our info sheet, we are only requesting 2 rows of info
                            // one row = one value
                            // so this works to extract since our first is always going to be the "Last updated" field
                            if (self.last_updated.is_empty()) {
                                self.last_updated = txt_values.first().unwrap_or(&base_string).clone();
                            } else {
                                self.season = txt_values.first().unwrap_or(&base_string).clone();
                            }
                        }
                    }
                }
            }
        }

        // player sheet parsing
        // Parse the player sheet and pull all relevant player info where possible
        let player_sheet_range = format!("{SHEET_PLAYER_LIST}!A2:E");
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
            if let Some(data) = player_sheet.data.as_ref() {
                for grid_data in data.iter() {
                    if let Some(row_data) = grid_data.row_data.as_ref() {
                        for row in row_data.iter() {
                            if let Some(cell_data) = row.values.as_ref() {
                                let bungie_name_cell = cell_data.get(0);
                                let discord_name_cell = cell_data.get(1);
                                let discord_id_cell = cell_data.get(2);
                                let bungie_membership_cell = cell_data.get(3);
                                let bungie_membership_platform_cell = cell_data.get(4);

                                let bungie_name = if let Some(bungie_name_cell) = bungie_name_cell {
                                    bungie_name_cell
                                        .formatted_value
                                        .as_ref()
                                        .unwrap_or(&base_string)
                                        .clone()
                                } else {
                                    base_string.clone()
                                };

                                let discord_name = if let Some(discord_name_cell) = discord_name_cell {
                                    discord_name_cell
                                        .formatted_value
                                        .as_ref()
                                        .unwrap_or(&base_string)
                                        .clone()
                                } else {
                                    base_string.clone()
                                };

                                let discord_id = if let Some(discord_id_cell) = discord_id_cell {
                                    discord_id_cell.formatted_value.as_ref().unwrap_or(&base_string).clone()
                                } else {
                                    base_string.clone()
                                };

                                let bungie_membership = if let Some(bungie_membership_cell) = bungie_membership_cell {
                                    bungie_membership_cell
                                        .formatted_value
                                        .as_ref()
                                        .unwrap_or(&base_string)
                                        .clone()
                                } else {
                                    base_string.clone()
                                };

                                let bungie_platform =
                                    if let Some(bugnie_platform_cell) = bungie_membership_platform_cell {
                                        bugnie_platform_cell
                                            .formatted_value
                                            .as_ref()
                                            .unwrap_or(&base_string)
                                            .clone()
                                    } else {
                                        base_string.clone()
                                    };
                                self.player_list
                                    .entry(bungie_membership.clone())
                                    .and_modify(|r| {
                                        *r = WorksheetPlayer {
                                            bungie_name: bungie_name.clone(),
                                            discord_name: discord_name.clone(),
                                            discord_id: discord_id.clone(),
                                            bungie_membership_id: bungie_membership.clone(),
                                            bungie_platform: bungie_platform.clone(),
                                        }
                                    })
                                    .or_insert(WorksheetPlayer {
                                        bungie_name: bungie_name.clone(),
                                        discord_name: discord_name.clone(),
                                        discord_id: discord_id.clone(),
                                        bungie_membership_id: bungie_membership.clone(),
                                        bungie_platform: bungie_platform.clone(),
                                    });
                            }
                        }
                    }
                }
            }
        }

        // now parse the clan sheets
        let mut clan_sheet_request = self.google.spreadsheets().get(&self.sheet_id);
        for clan_sheet in clan_sheet_names.iter() {
            let info_range = format!("'{clan_sheet}'!B1:B3");
            let roster_range = format!("'{clan_sheet}'!A6:F");
            clan_sheet_request = clan_sheet_request.add_ranges(&info_range).add_ranges(&roster_range);
        }

        let (_, clan_spreadsheet) = clan_sheet_request.include_grid_data(true).doit().await?;
        if let Some(clan_sheets) = clan_spreadsheet.sheets {
            for sheet in clan_sheets.iter() {
                if let Some(data) = sheet.data.as_ref() {
                    let mut clan_name = None;
                    let mut clan_group_id = None;
                    let mut clan_total_members = None;
                    let mut clan_members = HashMap::new();
                    for grid_data in data.iter() {
                        if let Some(row_data) = grid_data.row_data.as_ref() {
                            for row in row_data.iter() {
                                // parse the row here
                                let mut txt_values = Vec::new();
                                if let Some(cell_data) = row.values.as_ref() {
                                    txt_values.extend(
                                        cell_data
                                            .iter()
                                            .map(|v| v.formatted_value.as_ref().unwrap_or(&base_string).clone())
                                            .collect::<Vec<String>>(),
                                    );
                                }

                                if clan_name.is_none() {
                                    clan_name = Some(txt_values.first().unwrap_or(&base_string).clone());
                                } else if clan_group_id.is_none() {
                                    clan_group_id = Some(
                                        txt_values
                                            .first()
                                            .unwrap_or(&base_string)
                                            .clone()
                                            .parse::<i64>()
                                            .unwrap_or_default(),
                                    );
                                } else if clan_total_members.is_none() {
                                    clan_total_members = Some(txt_values.first().unwrap_or(&base_string).clone());
                                } else {
                                    let bungie_name = txt_values.first().unwrap_or(&base_string).clone();
                                    let bungie_id = txt_values
                                        .get(1)
                                        .unwrap_or(&base_string)
                                        .parse::<i64>()
                                        .unwrap_or_default();
                                    let role = txt_values
                                        .get(2)
                                        .unwrap_or(&base_string)
                                        .parse::<i64>()
                                        .unwrap_or_default();
                                    let last_online = txt_values.get(3).unwrap_or(&base_string).clone();
                                    let seasonal_activities = txt_values
                                        .get(4)
                                        .unwrap_or(&base_string)
                                        .parse::<i64>()
                                        .unwrap_or_default();

                                    let seasonal_activities_with_clan = txt_values
                                        .get(5)
                                        .unwrap_or(&base_string)
                                        .parse::<i64>()
                                        .unwrap_or_default();
                                    let mut seasonal_activities_with_clan_percent =
                                        txt_values.get(6).unwrap_or(&base_string).clone();

                                    // removed the % at the end if its visible

                                    if (seasonal_activities_with_clan_percent.ends_with("%")) {
                                        seasonal_activities_with_clan_percent
                                            .truncate(seasonal_activities_with_clan_percent.len() - 1);
                                    }

                                    let seasonal_activities_with_clan_percent =
                                        seasonal_activities_with_clan_percent.parse::<f64>().unwrap_or_default();

                                    let frequent_clan_members = txt_values.get(5).unwrap_or(&base_string).clone();

                                    clan_members.insert(
                                        bungie_id,
                                        WorksheetClanMember {
                                            name: bungie_name,
                                            bungie_id,
                                            role,
                                            last_online,
                                            seasonal_activities,
                                            seasonal_activities_with_clan,
                                            seasonal_activities_with_clan_percent,
                                            frequent_clan_members: frequent_clan_members
                                                .split(",")
                                                .map(|v| v.to_string())
                                                .collect::<Vec<String>>(),
                                        },
                                    );
                                }
                            }
                        }
                    }

                    // track
                    let clan_id = clan_group_id.unwrap_or_default();
                    self.clans.insert(
                        clan_id,
                        WorksheetClan {
                            name: clan_name.unwrap_or_default(),
                            group_id: clan_id,
                            members: clan_members,
                            members_sorted: Vec::new(),
                        },
                    );
                }
            }
        }

        // clans

        Ok(())
    }

    /// based off information already provided sync using the api
    pub async fn api_sync(&mut self, env: &Env) -> anyhow::Result<()> {
        let mut clan_group_id_strings = Vec::new();
        let mut clan_group_ids = Vec::new();
        for (clan_id, _) in self.clans.iter() {
            clan_group_id_strings.push(clan_id.to_string());
            clan_group_ids.push(*clan_id);
        }

        tracing::info!("Getting latest clan info based off spreadsheet clans");
        lib_destiny::jobs::clan::info(&clan_group_id_strings, env).await?;

        tracing::info!("Getting lastest clan roster info based off spreadsheet clans");
        lib_destiny::jobs::clan::roster(&clan_group_id_strings, env).await?;

        tracing::info!("Marking spreadsheet clans as network");
        lib_destiny::jobs::clan::make_network(&clan_group_id_strings, env).await?;

        tracing::info!("Syncing clan info and roster to local database");
        let mut app_state = AppState::new(env).await;
        for clan_id in clan_group_ids.iter() {
            let clan_info = lib_destiny::app::clan::get(*clan_id, &mut app_state).await;
            let clan_roster = lib_destiny::app::clan::get_roster(*clan_id, &mut app_state).await;

            tracing::info!("Syncing latest {clan_id} info to workbook");
            if let Some(clan_info) = clan_info {
                self.clans.entry(*clan_id).and_modify(|clan| {
                    clan.name = clan_info.name.clone();
                });
            }

            tracing::info!("Syncing latest {clan_id} roster to workbook");
            self.clans.entry(*clan_id).and_modify(|clan| {
                clan.members.clear();

                for member in clan_roster.iter() {
                    let membership_id = member.membership_id.to_string();
                    self.player_list
                        .entry(membership_id.clone())
                        .and_modify(|m| {
                            m.bungie_name = member.display_name_global.clone();
                            m.bungie_platform = member.platform.to_string();
                        })
                        .or_insert(WorksheetPlayer {
                            bungie_membership_id: membership_id.clone(),
                            bungie_name: member.display_name_global.clone(),
                            discord_id: String::new(),
                            discord_name: String::new(),
                            bungie_platform: member.platform.to_string(),
                        });

                    let last_played_at = chrono::DateTime::<chrono::Utc>::from_utc(
                        chrono::NaiveDateTime::from_timestamp_opt(member.last_played_at, 0).unwrap(),
                        chrono::Utc,
                    );
                    clan.members.insert(
                        member.membership_id,
                        WorksheetClanMember {
                            name: member.display_name_global.clone(),
                            bungie_id: member.membership_id,
                            role: member.clan_group_role,
                            last_online: format!("{}", last_played_at),
                            seasonal_activities: 0,
                            seasonal_activities_with_clan_percent: 0f64,
                            seasonal_activities_with_clan: 0,
                            frequent_clan_members: Vec::new(),
                        },
                    );
                }
            });
        }

        tracing::info!("Updating players discord information");
        for (player_id, player) in self.player_list.iter_mut() {
            if player.discord_id.trim().len() > 0 {
                tracing::info!("Fetching {} linked discord username", player.bungie_name);
                if false {
                    let member_data = discord::member_api(&player.discord_id, env, &app_state).await;
                    if let Some(member_data) = member_data {
                        let discriminator = member_data.discriminator.unwrap_or_default();

                        player.discord_name = if discriminator == "0" || discriminator.is_empty() {
                            member_data.username.unwrap_or(
                                member_data
                                    .display_name
                                    .unwrap_or(member_data.global_name.unwrap_or(player.discord_name.clone())),
                            )
                        } else {
                            format!(
                                "{}#{}",
                                member_data.username.unwrap_or(
                                    member_data
                                        .display_name
                                        .unwrap_or(member_data.global_name.unwrap_or(player.discord_name.clone()))
                                ),
                                discriminator
                            )
                        };
                    }

                    // sleep so we can avoid being rate limited
                    levelcrush::tokio::time::sleep(Duration::from_millis(1000)).await;
                }
            } else {
                tracing::info!("No known discord for  {}", player.bungie_name);
            }
        }

        // sort members
        let mut pl = self
            .player_list
            .iter()
            .map(|(membership_id, member_data)| member_data.clone())
            .collect::<Vec<WorksheetPlayer>>();

        pl.sort_by(|a, b| {
            let a_name = a.bungie_name.to_lowercase();
            let b_name = b.bungie_name.to_lowercase();
            let a_name = a_name.trim();
            let b_name = b_name.trim();
            a_name.cmp(b_name)
        });
        self.player_list_sorted = pl.into_iter().map(|v| v.bungie_membership_id).collect::<Vec<String>>();

        // sort clan members
        for (clan_id, clan_data) in self.clans.iter_mut() {
            let mut cl = clan_data
                .members
                .iter()
                .map(|(membership_id, member)| member.clone())
                .collect::<Vec<WorksheetClanMember>>();

            cl.sort_by(|a, b| {
                b.role.cmp(&a.role).then_with(|| {
                    let a_name = a.name.to_lowercase();
                    let b_name = b.name.to_lowercase();
                    let a_name = a_name.trim();
                    let b_name = b_name.trim();
                    a_name.cmp(b_name)
                })
            });
            clan_data.members_sorted = cl.into_iter().map(|data| data.bungie_id).collect::<Vec<i64>>();
        }

        Ok(())
    }

    pub async fn generate_reports(&mut self, env: &Env) -> anyhow::Result<()> {
        tracing::info!("Getting active seasons");
        let mut app_state = AppState::new(env).await;

        let target_season = self.season.to_lowercase();
        let workbook_season = target_season.trim();
        let target_season = if workbook_season.is_empty() || workbook_season == "lifetime" {
            0
        } else {
            self.season.parse::<i64>().unwrap_or_default()
        };

        tracing::info!("Running reports for season: {target_season}");

        // update the updated timestamp
        let current_timestamp = format!("{} UTC", chrono::Utc::now().format("%c"));
        tracing::info!("Updating 'last updated' field to: {}", current_timestamp);
        self.last_updated = current_timestamp;

        // update season if neccessary
        if target_season == 0 {
            self.season = "Lifetime".to_string();
        }

        let modes = vec![];
        let mut final_reports = HashMap::new();
        'task_loop: loop {
            for (membership_id, player_data) in self.player_list.iter() {
                let membership_id = membership_id.clone();
                if !final_reports.contains_key(&membership_id) {
                    let (timestamp, report) = match (target_season) {
                        0 => {
                            lib_destiny::app::report::member::lifetime(
                                membership_id.as_str(),
                                &modes,
                                true,
                                &mut app_state,
                            )
                            .await
                        }
                        default => {
                            lib_destiny::app::report::member::season(
                                membership_id.as_str(),
                                &modes,
                                target_season,
                                true,
                                &mut app_state,
                            )
                            .await
                        }
                    };

                    if let Some(report) = report {
                        tracing::info!("Storing {} into reports", membership_id);
                        final_reports.insert(membership_id, report);
                    }
                }
            }
            if final_reports.len() == self.player_list.len() {
                self.player_reports = final_reports;
                break 'task_loop;
            } else {
                app_state.priority_tasks.step().await;
                levelcrush::tokio::time::sleep(Duration::from_millis(1000)).await;
            }
        }

        tracing::info!(
            "Final Reports Generated: {} out of {}",
            self.player_reports.len(),
            self.player_list.len()
        );

        Ok(())
    }

    /// take the info from the local workbook and save it to the google spreadsheet
    pub async fn save(&self) -> anyhow::Result<()> {
        // define player zone ranges
        let mut clear_request = BatchClearValuesRequest::default();
        let mut player_zones = Vec::new();
        for (clan_id, clan_info) in self.clans.iter() {
            player_zones.push(format!("'[Clan] {}'!A6:F", clan_info.name));
        }
        player_zones.push(format!("{SHEET_PLAYER_LIST}!A2:E"));
        clear_request.ranges = Some(player_zones);

        tracing::info!("Clearing  bulk writable zones");
        self.google
            .spreadsheets()
            .values_batch_clear(clear_request, &self.sheet_id)
            .doit()
            .await?;

        let mut write_batch_request = BatchUpdateValuesRequest::default();
        let mut data_ranges = Vec::new();

        // extract player range values
        let mut player_list_values = Vec::new();
        for membership_id in self.player_list_sorted.iter() {
            let membership_id = membership_id.clone();
            let player = self
                .player_list
                .get(&membership_id)
                .expect("Should of been member data here");

            // row.push(vec![Value])
            player_list_values.push(vec![
                Value::String(player.bungie_name.clone()),
                Value::String(player.discord_name.clone()),
                Value::String(player.discord_id.clone()),
                Value::String(player.bungie_membership_id.clone()),
                Value::String(player.bungie_platform.clone()),
            ]);
        }

        // construct player range
        let mut player_value_range = ValueRange::default();
        player_value_range.range = Some(format!("{SHEET_PLAYER_LIST}!A2:E"));
        player_value_range.values = Some(player_list_values);
        data_ranges.push(player_value_range);

        // start building the clan ranges
        for (clan_id, clan) in self.clans.iter() {
            let mut clan_range = ValueRange::default();
            clan_range.range = Some(format!("'[Clan] {}'!A6:H", clan.name));

            let mut clan_values = Vec::new();
            for member_id in clan.members_sorted.iter() {
                let membership_id = member_id.to_string();
                let member = clan.members.get(member_id).expect("Expected clan member data here");
                let report = self.player_reports.get(&membership_id);
                let (last_played, activity_count, activity_count_clan, activity_percent_with_clan, frequent_clan_mates) =
                    if let Some(player_report) = report {
                        let last_played_datetime = chrono::DateTime::<chrono::Utc>::from_utc(
                            chrono::NaiveDateTime::from_timestamp_opt(player_report.last_played_at, 0).unwrap(),
                            chrono::Utc,
                        );

                        let seasonal_activities = player_report.activity_attempts;

                        let seasonal_activities_with_clan = player_report.activity_attempts_with_clan;
                        let seasonal_activities_with_clan_percent =
                            ((seasonal_activities_with_clan as f64 / seasonal_activities as f64) * 100.00f64);

                        // guard against NaN, in case of 0 / 0 for example
                        let seasonal_activities_with_clan_percent = if seasonal_activities_with_clan_percent.is_nan() {
                            0f64
                        } else {
                            seasonal_activities_with_clan_percent
                        };

                        let frequent_clan_members = player_report
                            .frequent_clan_members
                            .iter()
                            .take(3)
                            .map(|r| format!("{} ({})", r.display_name, r.activities))
                            .collect::<Vec<String>>()
                            .join(", ");

                        (
                            format!("{}", last_played_datetime),
                            seasonal_activities,
                            seasonal_activities_with_clan,
                            seasonal_activities_with_clan_percent,
                            frequent_clan_members,
                        )
                    } else {
                        ("N/A".to_string(), 0, 0, 0f64, "N/A".to_string())
                    };

                clan_values.push(vec![
                    Value::String(member.name.clone()),
                    Value::String(member_id.to_string()),
                    Value::String(member.role.to_string()),
                    Value::String(last_played),
                    Value::String(activity_count.to_string()),
                    Value::String(activity_count_clan.to_string()),
                    Value::String(format!("{}%", activity_percent_with_clan)),
                    Value::String(frequent_clan_mates),
                ]);
            }

            clan_range.values = Some(clan_values);
            data_ranges.push(clan_range);
        }

        // update Info sheet
        let mut info_range = ValueRange::default();
        info_range.range = Some("'Info'!B1:B2".to_string());
        info_range.values = Some(vec![
            vec![Value::String(self.last_updated.clone())],
            vec![Value::String(self.season.to_string())],
        ]);
        data_ranges.push(info_range);

        write_batch_request.data = Some(data_ranges);
        write_batch_request.value_input_option = Some("USER_ENTERED".to_string());

        tracing::info!("Writing to spreadsheet");
        self.google
            .spreadsheets()
            .values_batch_update(write_batch_request, &self.sheet_id)
            .doit()
            .await?;

        Ok(())
    }
}
