use std::time::Duration;

use crate::app::state::{AppState, Setting};
use crate::bungie::enums::DestinyActivityModeType;
use crate::env::AppVariable;
use crate::{app, database, env, routes};
use levelcrush::cache::CacheValue;
use levelcrush::server::Server;
use levelcrush::{tokio, tracing};

pub async fn run() {
    let server_port = env::get(AppVariable::ServerPort).parse::<u16>().unwrap_or(3003);

    let app_state = AppState::new().await;

    //println!("Listening Port (ENV): {}", server_port);
    //server::run(server_port).await;

    let mut app_state_bg = app_state.clone();
    let cache_task = tokio::spawn(async move {
        loop {
            app_state_bg.cache.prune().await;

            // all known app defiend groups will stay in cache. But this will catch any stray entries
            app_state_bg.leaderboards.prune().await;
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        }
    });

    let mut settings_app_state = app_state.clone();
    let settings_updater = tokio::spawn(async move {
        loop {
            // update modes into our setting cache
            let modes = database::setting::modes(&settings_app_state.database).await;
            settings_app_state
                .settings
                .write(
                    app::settings::CACHE_KEY_MODES,
                    CacheValue::persistant(Setting::Modes(modes)),
                )
                .await;

            // let the settings be updated every 5 minutes
            tokio::time::sleep(tokio::time::Duration::from_secs(300)).await;
        }
    });

    let mut season_app_state = app_state.clone();
    let seasons_updater = tokio::spawn(async move {
        loop {
            tracing::info!("Fetching season");
            let seasons = database::seasons::get_all_active(&season_app_state.database).await;
            season_app_state
                .seasons
                .write("active_seasons", CacheValue::persistant(seasons))
                .await;

            tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
        }
    });

    let mut leaderboard_state = app_state.clone();
    let leaderboard_updater = tokio::spawn(async move {
        let mut first_start = true;
        loop {
            tracing::info!("Updating leaderboards");

            let modes = if first_start {
                database::setting::modes(&leaderboard_state.database).await
            } else {
                app::settings::modes(&leaderboard_state).await
            };

            for group in modes.iter() {
                if group.leaderboard == 0 {
                    continue; // skip and move on
                }

                tracing::info!("Updating {} leaderboard", group.name);
                let (mut target_modes, results) = if group.name == "Raid" {
                    (vec![4], database::leaderboard::raids(&leaderboard_state.database).await)
                } else {
                    let group_modes = group
                        .value
                        .split(',')
                        .map(|v| DestinyActivityModeType::from(v) as i32)
                        .collect::<Vec<i32>>();
                    (
                        group_modes.clone(),
                        if group.name.to_lowercase().contains("pvp") {
                            database::leaderboard::pvp_based(&group_modes, &leaderboard_state.database).await
                        } else {
                            database::leaderboard::generic(&group_modes, &leaderboard_state.database).await
                        },
                    )
                };

                // sort so they are in a predicatable order
                target_modes.sort();

                let mode_str = target_modes
                    .into_iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(",");

                // this is entirely redudant but it makes life easier
                if mode_str == "4" {
                    leaderboard_state
                        .leaderboards
                        .write("Raid", CacheValue::persistant(results.clone()))
                        .await;
                }

                leaderboard_state
                    .leaderboards
                    .write(&mode_str, CacheValue::persistant(results))
                    .await;
            }

            tracing::info!("Updating Title leaderboard");
            let title_leaderboard = database::leaderboard::titles(&leaderboard_state.database).await;
            leaderboard_state
                .leaderboards
                .write("Titles", CacheValue::persistant(title_leaderboard))
                .await;

            first_start = false;
            tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
        }
    });

    let app_state_task = app_state.clone();
    let task_manager = tokio::spawn(async move {
        loop {
            let running = app_state_task.tasks.step().await;
            if running > 0 {
                tracing::info!("Running {} normal level task", running);
            }

            let running = app_state_task.priority_tasks.step().await;
            if running > 0 {
                tracing::info!("Running {} priority level task", running);
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });

    let rate_limit = std::env::var("RATE_LIMIT")
        .unwrap_or_default()
        .parse::<u64>()
        .unwrap_or(100);
    let rate_limit_per = std::env::var("RATE_LIMIT_DURATION")
        .unwrap_or_default()
        .parse::<u64>()
        .unwrap_or(10);

    let rate_limit_buffer = std::env::var("RATE_LIMIT_BUFFER")
        .unwrap_or_default()
        .parse::<u64>()
        .unwrap_or(1024);

    let server_task = tokio::spawn(async move {
        Server::new(server_port)
            .enable_cors()
            .enable_rate_limit(rate_limit, Duration::from_secs(rate_limit_per), rate_limit_buffer)
            .run(routes::router(), app_state)
            .await;
    });

    // run both  concurrently
    (_, _, _, _, _) = tokio::join!(
        server_task,
        cache_task,
        task_manager,
        settings_updater,
        leaderboard_updater
    );
}
