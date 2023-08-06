use crate::bungie::schemas::{DestinyProfileComponent, DestinyRecordComponentMap};
use crate::database;
use crate::database::member::MemberRecord;
use crate::database::triumph::MemberTriumphRecord;
use levelcrush::tracing;
use levelcrush::alias::destiny::MembershipId;
use levelcrush::alias::RecordId;
use levelcrush::util::unix_timestamp;
use sqlx::SqlitePool;

const CHUNK_SIZE_TRIUMPH: usize = 500;

/// sync a profile
pub async fn profile(profile: &DestinyProfileComponent, database: &SqlitePool) -> RecordId {
    let user_card = &profile.user_info;
    let membership_id = user_card.membership_id.parse::<i64>().unwrap_or_default();
    let membership_type = user_card.membership_type as i64;
    let global_display_name = format!(
        "{}#{:0>4}",
        user_card.global_display_name, user_card.global_display_name_code
    );

    tracing::info!(
        "Syncing Profile now...({} | {})",
        user_card.display_name,
        global_display_name
    );
    // create a member record structure to prepare for insertion or deletion
    let mut member = MemberRecord {
        id: 0,
        membership_id,
        platform: membership_type,
        display_name: user_card.display_name.clone(),
        display_name_global: global_display_name,
        guardian_rank_current: profile.guardian_rank_current as i64,
        guardian_rank_lifetime: profile.guardian_rank_lifetime as i64,
        last_played_at: profile.date_last_played.timestamp() as i64,
        created_at: 0,
        updated_at: 0,
        deleted_at: 0,
    };

    // determine the best way to update or insert our profile with our database
    tracing::info!("Attempting to find: {}", membership_id);
    let existing_record = database::member::get_record(membership_id, database).await;
    if let Some(existing_record) = existing_record {
        tracing::info!("Updating member record");
        // record found, update it!
        member.id = existing_record.id;
        member.created_at = existing_record.created_at;
        member.updated_at = unix_timestamp();
        member.deleted_at = existing_record.deleted_at;

        // send off to the database
        database::member::update(&member, database).await;

        // return member.id
        member.id
    } else {
        tracing::info!("Creating new profile record");
        // no record, insert it!
        member.id = 0;
        member.created_at = unix_timestamp();
        member.updated_at = 0;
        member.deleted_at = 0;

        // send off to the database
        // technically sqlx will return a u64 for an id, but our database record primary id is a i32
        database::member::create(member, database).await
    }
}

pub async fn triumphs(membership_id: MembershipId, data: &DestinyRecordComponentMap, pool: &SqlitePool) {
    let records = data
        .iter()
        .map(|(hash, triumph)| {
            let hash = hash.parse::<i64>().unwrap_or_default();
            MemberTriumphRecord {
                id: 0,
                membership_id,
                hash,
                state: triumph.state as i64,
                times_completed: triumph.completed_count as i64,
                created_at: unix_timestamp(),
                updated_at: 0,
                deleted_at: 0,
            }
        })
        .collect::<Vec<MemberTriumphRecord>>();

    tracing::info!("Writing triumph information to db for {}", membership_id);
    for chunk in records.chunks(CHUNK_SIZE_TRIUMPH) {
        database::triumph::member_write(chunk, pool).await;
    }
}
