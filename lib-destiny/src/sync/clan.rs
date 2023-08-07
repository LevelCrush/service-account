use crate::bungie::schemas::{GroupMember, GroupV2};
use crate::database;
use crate::database::clan::{ClanMemberRecord, ClanRecord};
use levelcrush::alias::{destiny::GroupId, destiny::MembershipId, destiny::MembershipType, UnixTimestamp};
use levelcrush::tracing;
use levelcrush::util::{slugify, unix_timestamp};
use sqlx::SqlitePool;
use std::collections::HashMap;
pub async fn info(group: &GroupV2, pool: &SqlitePool) {
    let group_id = group.group_id.parse::<GroupId>().unwrap_or_default();

    tracing::info!("Searching for clan: {}", group_id);

    let existing_clan = database::clan::get(group_id, pool).await;

    let mut clan_record = ClanRecord {
        group_id,
        name: group.name.clone(),
        slug: slugify(group.name.as_str()),
        motto: group.motto.clone(),
        about: group.about.clone(),
        call_sign: group.clan_info.clan_callsign.clone(),
        created_at: unix_timestamp(),
        updated_at: 0,
        deleted_at: 0,
        id: 0,
        is_network: 0,
    };

    if let Some(existing_clan) = existing_clan {
        clan_record.id = existing_clan.id;
        clan_record.created_at = existing_clan.created_at;
        clan_record.updated_at = unix_timestamp();
        clan_record.deleted_at = existing_clan.deleted_at;
        clan_record.is_network = existing_clan.is_network;

        tracing::warn!("Updating data for clan: {}", group_id);
        database::clan::update(&clan_record, pool).await;
    } else {
        clan_record.id = 0;
        clan_record.created_at = unix_timestamp();
        clan_record.updated_at = 0;
        clan_record.deleted_at = 0;
        clan_record.is_network = 0;

        tracing::warn!("Inserting data for clan: {}", group_id);
        database::clan::create(clan_record, pool).await;
    }
}

/// sync an entire clan roster
/// note: this assumes the passed group members are the entire clan
/// returns a hashmap  in the format of (key = membership_id: i64, value = membership_platform: i32)
pub async fn roster(
    group_id: GroupId,
    members: &[GroupMember],
    pool: &SqlitePool,
) -> HashMap<MembershipId, MembershipType> {
    // this is important for later on
    // we will use this array to determine who is n

    let membership_ids: Vec<MembershipId> = members
        .iter()
        .map(|member| {
            member
                .user_info
                .membership_id
                .parse::<MembershipId>()
                .unwrap_or_default()
        })
        .collect();

    // get members SPECIFICALLY in this clan
    let mut clan_existing_members = database::clan::existing_members(group_id, pool).await;
    let mut clan_roster = HashMap::with_capacity(members.len());

    tracing::warn!("In Clan Before Update: {}", clan_existing_members.len());

    // this is for any member in the table (people can between clans)
    tracing::warn!("Searching for {} total membership_ids", membership_ids.len());
    let existing_members = database::clan::find_by_membership(&membership_ids, pool).await;

    tracing::warn!("Found {} total", existing_members.len());

    // loop through the supplied members and try to find matches
    let mut update_members = Vec::new();
    let mut new_members = Vec::new();
    for member in members.iter() {
        let membership_id = member
            .user_info
            .membership_id
            .parse::<MembershipId>()
            .unwrap_or_default();

        let membership_type = member.user_info.membership_type as MembershipType;
        let record_id = match existing_members.get(&membership_id) {
            Some(record) => *record,
            _ => 0,
        };

        let record = ClanMemberRecord {
            group_id,
            group_role: member.member_type as i64,
            membership_id,
            platform: membership_type,
            joined_at: member.join_date.timestamp() as UnixTimestamp,
            created_at: unix_timestamp(),
            updated_at: unix_timestamp(),
            deleted_at: 0, // this field is basically a dud for this case. Or at least. At the moment it is
            id: record_id,
        };

        // choose where to send this off too
        if record_id > 0 {
            update_members.push(record);
        } else {
            new_members.push(record);
        }

        // now add them into the clan existing member  pool, if we are updating them or creating them we simply set their record id to 0
        // when we go to remove clan members, any membership id here with a record id that is non zero should be deleted
        clan_existing_members
            .entry(membership_id)
            .and_modify(|v| *v = 0)
            .or_insert(0);

        clan_roster.entry(membership_id).or_insert(membership_type);
    }

    // insert each member
    tracing::warn!(
        "Inserting new members into clan_members table: {} total",
        new_members.len()
    );
    for member in new_members {
        database::clan::add_member(member, pool).await;
    }

    // update each member individually, no, this is not the most efficient
    tracing::warn!(
        "Updating members in the clan member table: {} total",
        update_members.len()
    );
    for member in &update_members {
        database::clan::update_member(member, pool).await;
    }

    // now remove any member not in our clan
    let mut target_ids = Vec::new();
    for (_, record_id) in clan_existing_members.iter() {
        if *record_id > 0 {
            // any record that has an id that is non zero should be removed. This means we never touched them
            target_ids.push(*record_id);
        }
    }

    if !target_ids.is_empty() {
        // remove clan members
        tracing::warn!("Removing clan members not in clan: {} total", target_ids.len());
        database::clan::remove_members(&target_ids, pool).await;
    }

    clan_roster
}
