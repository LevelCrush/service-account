use levelcrush::{anyhow, tokio, tracing};
use lib_destiny::app::state::AppState;
use lib_destiny::env::{AppVariable, Env};

use crate::drive::DriveDestinyReports;
use crate::jobs::sheets::discord_sync;
use crate::sheets::MasterWorkbook;

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
    let mut create_sheets = Vec::new();
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

            if drive
                .get_season_clan_overview(season.number as i32, clan.group_id)
                .is_none()
            {
                create_sheets.push((season.number as i32, clan.group_id, clan.name.clone()));
            } else {
                tracing::info!("Oversheet for Season {} | {} found", season.number, clan.name);
            }
        }
    }

    for (season, group_id, clan_name) in create_sheets.into_iter() {
        tracing::warn!("Overview sheet not found: Season {} | {}", season, clan_name);
        drive.api_copy_overview(season, group_id, clan_name).await?;

        tokio::time::sleep(tokio::time::Duration::from_secs(7)).await;
    }

    tracing::info!("Done copying over and setting up report folder. Now performing api syncs");
    drop(drive);

    tracing::info!("Rescanning drive before api sync");
    let mut drive = DriveDestinyReports::get(&master_folder).await?;
    drive.load().await?;

    // update lifetime overview first
    if let Some(overview_id) = drive.google_workbooks.get("overview") {
        tracing::warn!("Loading Lifetime - Overview workbook");
        let mut workbook = MasterWorkbook::connect(&overview_id).await?;
        workbook.load().await?;

        tracing::warn!("Attempting api sync on lifetime overview");
        workbook.api_sync(env).await?;

        tracing::warn!("Generating lifetime reports..this can take some time");
        workbook.generate_reports(env).await?;

        tracing::warn!("Saving lifetime overview");
        workbook.save().await?;
    }

    for (_, drive_season) in drive.seasons.iter() {
        for (_, drive_clan) in drive_season.clans.iter() {
            if let Some(workbook_id) = drive_clan.google_workbooks.get("overview") {
                let formatted_name = format!("Season {} | {} Overview", drive_season.number, drive_clan.name);
                tracing::warn!("{} is being loaded", formatted_name);

                let mut workbook = MasterWorkbook::connect(workbook_id).await?;
                workbook.load().await?;

                tracing::warn!("Attempting api sync on {}", formatted_name);
                workbook.api_sync(env).await?;

                tracing::warn!("Generating {} player reports..this can take some time", formatted_name);
                workbook.generate_reports(env).await?;

                tracing::warn!("Saving {}", formatted_name);
                let r = workbook.save().await;
                if r.is_err() {
                    tracing::info!("Retrying in 30s");
                    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
                    tracing::warn!("Saving {}", formatted_name);
                    workbook.save().await;
                }
                tracing::info!("Throttling 10s to avoid resource limitations");
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            }

            // loop through players and update worksheets
            for (player_drive_id, drive_player) in drive_clan.players.iter() {
                if let Some(workbook_id) = drive_player.google_workbooks.get("overview") {
                    let formatted_name = format!(
                        "Season {} | {} Overview | {} Overview",
                        drive_season.number, drive_clan.name, drive_player.bungie_name
                    );
                    tracing::warn!("Updating {}({})", formatted_name, workbook_id);
                }
            }
        }
    }

    tracing::info!("Syncing discord");
    discord_sync(env).await?;

    tracing::info!("Done!");

    Ok(())
}
