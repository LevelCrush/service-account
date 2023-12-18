use levelcrush::{anyhow, tracing};
use lib_destiny::app::state::AppState;
use lib_destiny::env::{AppVariable, Env};

use crate::drive::DriveDestinyReports;

pub async fn generate(args: &[String], env: &Env) -> anyhow::Result<()> {
    let mut app_state = AppState::new(env).await;

    tracing::info!("Generating report for");
    // parse arguments as group ids
    let base_string = String::new();
    let season = args.first().unwrap_or(&base_string).clone();
    let season = season.to_lowercase();
    let season = season.trim();

    // scan google drive for already existing files
    tracing::info!("Scanning for existing google sheets");
    let master_folder = env.get(AppVariable::GoogleDriveReportDestinyFolder);
    let mut drive = DriveDestinyReports::get(&master_folder).await?;
    drive.load().await?;

    tracing::info!("Querying database for seasons that need to be tracked");
    let seasons = lib_destiny::database::seasons::get_all_active(&app_state.database).await;
    for season in seasons.iter() {
        if drive.get_season(season.number as i32).is_none() {
            tracing::warn!("Creating Folder `Season {}` in google drive", season.number);
            drive.api_create_season(season.number as i32).await?;
        } else {
            tracing::info!("Existing Folder `Season {}` in google drive", season.number);
        }
    }

    tracing::info!("Querying database for clans that need to be tracked");
    let network_clans = lib_destiny::app::clan::network(&mut app_state).await;
    for clan in network_clans.iter() {
        for season in seasons.iter() {
            if drive.get_season_clan(season.number as i32, clan.group_id).is_none() {
                tracing::warn!(
                    "Creating Folder `{} [{}]` in `Season {}`",
                    clan.name,
                    clan.group_id,
                    season.number
                );
                drive
                    .api_create_season_clan(season.number as i32, clan.group_id, clan.name.clone())
                    .await?;
            } else {
                tracing::info!(
                    "Existing Folder `{} [{}]` in `Season {}`",
                    clan.name,
                    clan.group_id,
                    season.number
                )
            }
        }
    }

    Ok(())
}
