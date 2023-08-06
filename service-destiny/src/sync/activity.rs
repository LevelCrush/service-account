use crate::{
    bungie::schemas::{DestinyHistoricalStatsPeriodGroup, DestinyPostGameCarnageReportData},
    database::{
        self,
        activity_history::ActivityHistoryRecord,
        activity_stats::ActivityStatRecord,
        instance::{InstanceMemberRecord, InstanceRecord},
    },
};
use levelcrush::tracing;
use levelcrush::{
    alias::{destiny::CharacterId, destiny::InstanceId, destiny::MembershipId, destiny::MembershipType, UnixTimestamp},
    util::unix_timestamp,
    SqlitePool,
};
use std::collections::{HashMap, HashSet};

const CHUNK_SIZE_ACTIVITIES: usize = 1000;
const CHUNK_SIZE_STATS: usize = 1000;
const CHUNK_SIZE_PARTY: usize = 1000;

/// syncs the activity history response and syncs to the database
/// returns a vector of instance ids
pub async fn history(
    membership_id: MembershipId,
    _membership_type: MembershipType,
    character_id: CharacterId,
    values: &[DestinyHistoricalStatsPeriodGroup],
    pool: &SqlitePool,
) -> Vec<InstanceId> {
    // pass 1 loop through our values and get the instance id's directly and store them off
    let instance_ids = {
        // best case of every activity history we get is going to be unique and we need to allocate
        let mut temp_hash = HashSet::with_capacity(values.len());
        for activity_history in values.iter() {
            temp_hash.insert(
                activity_history
                    .details
                    .instance_id
                    .parse::<InstanceId>()
                    .unwrap_or_default(),
            );
        }
        temp_hash.into_iter().collect::<Vec<i64>>()
    };

    // now loop through the history and genereate records
    let records = values
        .iter()
        .map(|activity_history| {
            // bungie api returns all in64 as a string format, we convert manually on our end to what it is intended to be
            // either we get the record id that matches this instance or set to 0
            let instance_id = activity_history
                .details
                .instance_id
                .parse::<InstanceId>()
                .unwrap_or_default();

            ActivityHistoryRecord {
                id: 0,
                membership_id,
                character_id,
                instance_id,
                activity_hash: activity_history.details.reference_id as i64,
                activity_hash_director: activity_history.details.director_activity_hash as i64,
                mode: activity_history.details.mode as i64,
                modes: activity_history
                    .details
                    .modes
                    .iter()
                    .map(|m| (*m as i32).to_string())
                    .collect::<Vec<String>>()
                    .join(","),
                platform_played: activity_history.details.membership_type as i64,
                private: activity_history.details.is_private as i64,
                occurred_at: activity_history.period.timestamp() as i64,
                created_at: unix_timestamp(),
                updated_at: 0,
                deleted_at: 0,
            }
        })
        .collect::<Vec<ActivityHistoryRecord>>();

    // chunk and then send
    for chunk in records.chunks(CHUNK_SIZE_ACTIVITIES) {
        database::activity_history::write(chunk, pool).await;
    }

    instance_ids
}

pub async fn stats(
    membership_id: MembershipId,
    _membership_type: MembershipType,
    character_id: CharacterId,
    values: &[DestinyHistoricalStatsPeriodGroup],
    pool: &SqlitePool,
) {
    let mut records = Vec::new();
    for stat_group in values.iter() {
        let instance_id = stat_group.details.instance_id.parse::<InstanceId>().unwrap_or_default();

        for (_, stat) in stat_group.values.iter() {
            records.push(ActivityStatRecord {
                id: 0,
                membership_id,
                character_id,
                activity_hash: stat_group.details.reference_id,
                activity_hash_director: stat_group.details.director_activity_hash,
                instance_id,
                name: stat.stat_id.clone(),
                value: stat.basic.value,
                value_display: stat.basic.display_value.clone(),
                created_at: unix_timestamp(),
                updated_at: 0,
                deleted_at: 0,
            });
        }
    }

    for chunk in records.chunks(CHUNK_SIZE_STATS) {
        database::activity_stats::write(chunk, pool).await;
    }
}

/// syncs our post game carnage report data to our database
/// returns a hash map of our key = (membership_id, membership_type aka the platform) and value = Vec<character_ids>
pub async fn instance(
    data: &DestinyPostGameCarnageReportData,
    pool: &SqlitePool,
) -> HashMap<(MembershipId, MembershipType), Vec<CharacterId>> {
    // parse instance id from the api
    let instance_id = data.details.instance_id.parse::<MembershipId>().unwrap_or_default();

    tracing::info!("Syncing activity history for: {}", instance_id);

    // scan through data to determin
    let mut activity_completed = false;
    let mut completion_reasons = HashMap::new();
    let mut instance_member_records = Vec::new();
    let mut instance_member_map = HashMap::new();
    for instance_data in data.entries.iter() {
        let membership_id = instance_data
            .player
            .user_info
            .membership_id
            .parse::<MembershipId>()
            .unwrap_or_default();

        let character_id = instance_data.character_id.parse::<CharacterId>().unwrap_or_default();
        let membership_type = instance_data.player.user_info.membership_type as MembershipType;

        // find out if the player completed
        let player_completed = match instance_data.values.get("completed") {
            Some(inst_value) => inst_value.basic.display_value == "Yes",
            _ => false,
        };

        // get the completion reason if possible
        let complete_reason_str = match instance_data.values.get("completionReason") {
            Some(inst_value) => inst_value.basic.display_value.as_str(),
            _ => "",
        };

        instance_member_records.push(InstanceMemberRecord {
            instance_id,
            membership_id,
            character_id,
            platform: membership_type,
            class_name: instance_data.player.character_class.clone(),
            class_hash: instance_data.player.class_hash as i64,
            emblem_hash: instance_data.player.emblem_hash as i64,
            light_level: instance_data.player.light_level as i64,
            clan_name: instance_data.player.clan_name.clone(),
            clan_tag: instance_data.player.clan_tag.clone(),
            completed: player_completed as i64,
            completion_reason: complete_reason_str.to_string(),
            created_at: unix_timestamp(),
            updated_at: 0,
            deleted_at: 0,
            id: 0,
        });

        // if at least one player completed, mark the activity as completed.
        // when we sync character activity history we can always get the specifics
        if player_completed {
            activity_completed = true;
        }

        // track to se if we have completed. Note, we only care about the first "completion" reason for whatever comes first
        // for the purpose of this sync we don't need to know **every** player reason
        // that is already handled by character activity history sync
        completion_reasons
            .entry(complete_reason_str)
            .or_insert(player_completed);

        instance_member_map
            .entry((membership_id, membership_type))
            .and_modify(|chars: &mut Vec<CharacterId>| chars.push(character_id))
            .or_insert(vec![character_id]);
    }

    let completed_reasons = completion_reasons
        .keys()
        .map(|reason| reason.to_string())
        .collect::<Vec<String>>()
        .join(",");

    let instance_record = InstanceRecord {
        instance_id,
        occurred_at: data.period.timestamp() as UnixTimestamp,
        starting_phase_index: data.starting_phase_index.unwrap_or_default() as i64,
        started_from_beginning: data.started_from_beginning.unwrap_or_default() as i64,
        activity_hash: data.details.reference_id as i64,
        activity_director_hash: data.details.director_activity_hash as i64,
        is_private: data.details.is_private as i64,
        completed: activity_completed as i64,
        completion_reasons: completed_reasons,
        created_at: unix_timestamp(),
        updated_at: 0,
        deleted_at: 0,
        id: 0,
    };

    // write instance
    database::instance::write(&instance_record, pool).await;

    // now write the members
    for chunk in instance_member_records.chunks(CHUNK_SIZE_PARTY) {
        database::instance::write_members(chunk, pool).await;
    }

    instance_member_map
}
