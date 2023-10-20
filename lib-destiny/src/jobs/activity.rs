use crate::app::state::AppState;
use crate::database;
use crate::env::Env;
use crate::jobs::task;
use levelcrush::anyhow;
use levelcrush::tracing;
use levelcrush::util::unix_timestamp;

pub async fn history(args: &[String], env: &Env) -> anyhow::Result<()> {
    tracing::info!("Running member character activity");
    tracing::info!("Setting up application state");
    let state = AppState::new(env).await;

    for bungie_name in args.iter() {
        let profile_results = task::profile_search(bungie_name, &state).await?;
        let (membership_id, membership_type, character_ids) = match profile_results {
            Some(profile_result) => (
                profile_result.membership_id,
                profile_result.membership_type,
                profile_result.characters,
            ),
            _ => (0, 0, Vec::new()),
        };

        // now run our character task
        for character_id in character_ids.iter() {
            task::activities(membership_id, membership_type, *character_id, false, &state).await?;
        }
    }

    Ok(())
}

/// crawl the instances that have available based off membership_activities
pub async fn crawl_instances(args: &[String], env: &Env) -> anyhow::Result<()> {
    tracing::info!("Crawling missing instance data");
    tracing::info!("Setting up application state");

    let total_instances = {
        if !args.is_empty() {
            match args.first() {
                Some(v) => v.parse::<usize>().unwrap_or_default(),
                _ => 1000,
            }
        } else {
            1000
        }
    };

    let state = AppState::new(env).await;

    // our starting record
    tracing::info!("Getting oldest record");
    let earliest_timestamp = 1498881600; // July 1st, 2017, Destiny 2 technically released on the 6th of September 2017, so this should cover fairly well

    tracing::info!("Getting most recent record");
    let recent_record = database::activity_history::get_recent(&state.database).await;
    let max_timestamp = match recent_record {
        Some(record) => record.occurred_at + 1, // whatever our most recent record occurred_at stamp is, + 1 second
        _ => unix_timestamp() - 3600,           // whatever the current time is of running this program - one hour
    };

    // work backwords
    let mut total_records = 0;
    let mut end_timestamp = max_timestamp;
    let mut start_timestamp = (end_timestamp - 2764800).max(earliest_timestamp);

    while start_timestamp > earliest_timestamp && total_records < total_instances {
        tracing::info!("Querying between timestamps: {} | {}", start_timestamp, end_timestamp);
        let missing_instances = database::activity_history::missing_instance_data(
            start_timestamp,
            end_timestamp,
            total_instances as i64,
            &state.database,
        )
        .await;

        tracing::info!(
            "Total Processed: {} | Incoming: {}",
            total_records,
            missing_instances.len()
        );
        total_records += missing_instances.len();

        for instances in missing_instances.chunks(100) {
            task::instance_data(instances, &state).await?;
        }

        // now make our end timestamp = what our current start timestamp is. On the next iteration this will decrement our current start timestamp by whatever amount we want.
        end_timestamp = start_timestamp;
        start_timestamp = start_timestamp.min(end_timestamp - 2764800).max(earliest_timestamp);
    }

    Ok(())
}

pub async fn instance_member_profiles(args: &[String], env: &Env) -> anyhow::Result<()> {
    let amount = {
        if !args.is_empty() {
            match args.first() {
                Some(v) => v.parse::<usize>().unwrap_or_default(),
                _ => 1000,
            }
        } else {
            1000
        }
    };

    let state = AppState::new(env).await;

    // our starting record
    tracing::info!("Getting oldest record");
    let earliest_timestamp = 1498881600; // Just 1st, 2017, Destiny 2 technically released on the 6th of September 2017, so this should cover fairly well

    tracing::info!("Getting most recent record");
    let recent_record = database::instance::get_recent(&state.database).await;
    let max_timestamp = match recent_record {
        Some(record) => record.occurred_at + 1, // whatever recent occurred at is + 1 second
        _ => unix_timestamp() - 3600,           // whatever the current time is of running this program - one hour
    };

    // work backwords
    let mut total_records = 0;
    let mut end_timestamp = max_timestamp;
    let mut start_timestamp = (end_timestamp - 2764800).max(earliest_timestamp);
    while start_timestamp > earliest_timestamp && total_records < amount {
        tracing::info!("Querying between timestamps: {} | {}", start_timestamp, end_timestamp);
        let need_profiles = database::instance::get_members_missing_profile(
            start_timestamp,
            end_timestamp,
            amount as i64,
            &state.database,
        )
        .await;

        tracing::info!("Total Processed: {} | Incoming: {}", total_records, need_profiles.len());
        total_records += need_profiles.len();

        // run our profile task
        for profile in need_profiles.iter() {
            task::profile(profile.membership_id, profile.platform, &state).await?;
        }

        // now make our end timestamp = what our current start timestamp is. On the next iteration this will decrement our current start timestamp by whatever amount we want.
        end_timestamp = start_timestamp;
        start_timestamp = start_timestamp.min(end_timestamp - 2764800).max(earliest_timestamp);
    }

    Ok(())
}
