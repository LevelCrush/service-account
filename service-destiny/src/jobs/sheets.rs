use google_sheets4::api::{BatchClearValuesRequest, BatchUpdateValuesRequest, ValueRange};
use google_sheets4::hyper::Client;
use google_sheets4::oauth2::{self};
use google_sheets4::{hyper, hyper_rustls, Sheets};
use google_sheets4::{
    hyper::client::HttpConnector, hyper_rustls::HttpsConnector, oauth2::authenticator::Authenticator,
};
use levelcrush::proc_macros::ExternalAPIResponse;
use levelcrush::{anyhow, project_str, tracing};
use lib_destiny::app::state::AppState;
use lib_destiny::env::{AppVariable, Env};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;

const GOOGLE_CREDENTIALS: &str = project_str!("google_credentials.json");

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
pub struct WorksheetClan {
    pub name: String,
    pub group_id: i64,
    pub members: Vec<(String, i64)>,
}

#[derive(Clone)]
pub struct MasterWorkbook {
    pub sheet_id: String,
    pub player_list: HashMap<String, WorksheetPlayer>,
    pub clans: HashMap<i64, WorksheetClan>,
    pub google: Sheets<HttpsConnector<HttpConnector>>,
}

#[ExternalAPIResponse]
pub struct DiscordUserResponse {
    pub id: Option<String>,
    pub username: Option<String>,
    pub discriminator: Option<String>,
    pub avatar: Option<String>,
    pub global_name: Option<String>,
    pub display_name: Option<String>,
}

/// queries a discord user directly by their discord id
pub async fn member_api(discord_id: &str, env: &Env, state: &AppState) -> Option<DiscordUserResponse> {
    let bot_token = env.get(AppVariable::DiscordBotToken);
    let bot_auth = format!("Bot {}", bot_token);
    let discord_user_id = discord_id;

    let endpoint = format!("https://discord.com/api/v10/users/{}", discord_user_id);
    let request = state
        .bungie
        .http_client
        .get(&endpoint)
        .header("Authorization", bot_auth)
        .send()
        .await;

    if let Ok(request) = request {
        let json = request.json::<DiscordUserResponse>().await;
        if let Ok(json) = json {
            Some(json)
        } else {
            let err = json.err().unwrap();
            tracing::error!("Error occurred while parsing user response:\r\n{}", err);
            None
        }
    } else {
        None
    }
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
            clans: HashMap::new(),
            google,
        };

        Ok(workbook)
    }

    /// this will populate our Masterworkbook data structure with data from the spreadsheet
    /// this will make 0 api calls to the bungie api
    /// use this function to provide a state that is READ FROM THE SPREADSHEET
    pub async fn load(&mut self) -> anyhow::Result<()> {
        // clear arrays
        self.clans.clear();
        self.player_list.clear();

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
        let base_string = String::new();
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
            let roster_range = format!("'{clan_sheet}'!A6:B");
            clan_sheet_request = clan_sheet_request.add_ranges(&info_range).add_ranges(&roster_range);
        }

        let (_, clan_spreadsheet) = clan_sheet_request.include_grid_data(true).doit().await?;
        if let Some(clan_sheets) = clan_spreadsheet.sheets {
            for sheet in clan_sheets.iter() {
                if let Some(data) = sheet.data.as_ref() {
                    let mut clan_name = None;
                    let mut clan_group_id = None;
                    let mut clan_total_members = None;
                    let mut clan_members = Vec::new();
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
                                    clan_members.push((
                                        txt_values.first().unwrap_or(&base_string).clone(),
                                        txt_values
                                            .last()
                                            .unwrap_or(&base_string)
                                            .clone()
                                            .parse::<i64>()
                                            .unwrap_or(0),
                                    ));
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
                    clan.members
                        .push((member.display_name_global.clone(), member.clan_group_role));
                }
            });
        }

        tracing::info!("Updating players discord information");
        for (player_id, player) in self.player_list.iter_mut() {
            if player.discord_id.trim().len() > 0 {
                tracing::info!("Fetching {} linked discord username", player.bungie_name);
                let member_data = member_api(&player.discord_id, env, &app_state).await;
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
            } else {
                tracing::info!("No known discord for  {}", player.bungie_name);
            }
        }

        Ok(())
    }

    /// take the info from the local workbook and save it to the google spreadsheet
    pub async fn save(&self) -> anyhow::Result<()> {
        // define player zone ranges
        let mut clear_request = BatchClearValuesRequest::default();
        let mut player_zones = Vec::new();
        for (clan_id, clan_info) in self.clans.iter() {
            player_zones.push(format!("'[Clan] {}'!A6:B", clan_info.name));
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
        let mut player_list_values = Vec::new();
        for (membership_id, player) in self.player_list.iter() {
            // row.push(vec![Value])
            player_list_values.push(vec![
                Value::String(player.bungie_name.clone()),
                Value::String(player.discord_name.clone()),
                Value::String(player.discord_id.clone()),
                Value::String(player.bungie_membership_id.clone()),
                Value::String(player.bungie_platform.clone()),
            ]);
        }
        let mut player_value_range = ValueRange::default();
        player_value_range.range = Some(format!("{SHEET_PLAYER_LIST}!A2:E"));
        player_value_range.values = Some(player_list_values);

        let mut data_ranges = Vec::new();
        for (clan_id, clan) in self.clans.iter() {
            let mut clan_range = ValueRange::default();
            clan_range.range = Some(format!("'[Clan] {}'!A6:B", clan.name));

            let mut clan_values = Vec::new();

            for (member_name, member_role) in clan.members.iter() {
                clan_values.push(vec![
                    Value::String(member_name.clone()),
                    Value::String(member_role.to_string()),
                ]);
            }

            clan_range.values = Some(clan_values);
            data_ranges.push(clan_range);
        }
        data_ranges.push(player_value_range);

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

pub async fn sync(env: &Env) -> anyhow::Result<()> {
    tracing::info!("Constructing workbook connection");
    let sheet_id = env.get(AppVariable::MasterWorkSheet);
    let mut workbook = MasterWorkbook::get(&sheet_id).await?;

    tracing::info!("Hydrating information");
    workbook.load().await?;

    tracing::info!("Updating from API");
    workbook.api_sync(env).await?;

    tracing::info!("Saving workbook");
    workbook.save().await?;

    Ok(())
}

pub async fn discord_sync(env: &Env) -> anyhow::Result<()> {
    Ok(())
}
