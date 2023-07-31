create table if not exists clan_members
(
    id            integer not null
        constraint clan_members_pk
            primary key autoincrement,
    group_id      integer not null,
    group_role    integer not null,
    membership_id integer not null,
    platform      integer not null,
    joined_at     integer not null,
    created_at    integer not null,
    updated_at    integer not null,
    deleted_at    integer not null,
    constraint clan_members_pk2
        unique (group_id, group_role, membership_id)
)
    strict;

create index if not exists clan_members_group_id_group_role_index
    on clan_members (group_id, group_role);

create index if not exists clan_members_group_id_index
    on clan_members (group_id);

create index if not exists clan_members_group_role_index
    on clan_members (group_role);

create index if not exists clan_members_joined_at_index
    on clan_members (joined_at);

create index if not exists clan_members_membership_id_index
    on clan_members (membership_id);

create table if not exists clans
(
    id         integer not null
        constraint clans_pk
            primary key autoincrement,
    group_id   integer not null
        constraint clans_pk2
            unique,
    name       text    not null,
    slug       text    not null,
    is_network integer not null,
    motto      text    not null,
    about      text    not null,
    call_sign  text    not null,
    created_at integer not null,
    updated_at integer not null,
    deleted_at integer not null
)
    strict;

create index if not exists clans_group_id_index
    on clans (group_id);

create index if not exists clans_is_network_index
    on clans (is_network);

create index if not exists clans_name_index
    on clans (name);

create index if not exists clans_slug_index
    on clans (slug);

create table if not exists instance_members
(
    id                integer not null
        constraint instance_members_pk
            primary key autoincrement,
    instance_id       integer not null,
    membership_id     integer not null,
    platform          integer not null,
    character_id      integer not null,
    class_hash        integer not null,
    class_name        TEXT    not null,
    emblem_hash       integer not null,
    light_level       integer not null,
    clan_name         TEXT    not null,
    clan_tag          TEXT    not null,
    completed         integer not null,
    completion_reason integer not null,
    created_at        integer not null,
    updated_at        integer not null,
    deleted_at        integer not null,
    constraint instance_members_pk2
        unique (membership_id, instance_id, character_id)
)
    strict;

create index if not exists instance_members_character_id_completed_index
    on instance_members (character_id, completed);

create index if not exists instance_members_character_id_index
    on instance_members (character_id);

create index if not exists instance_members_class_hash_index
    on instance_members (class_hash);

create index if not exists instance_members_class_name_completed_index
    on instance_members (class_name, completed);

create index if not exists instance_members_class_name_index
    on instance_members (class_name);

create index if not exists instance_members_completed_index
    on instance_members (completed);

create index if not exists instance_members_completion_reason_index
    on instance_members (completion_reason);

create index if not exists instance_members_emblem_hash_index
    on instance_members (emblem_hash);

create index if not exists instance_members_instance_id_completed_index
    on instance_members (instance_id, completed);

create index if not exists instance_members_instance_id_completed_membership_id_index
    on instance_members (instance_id, completed, membership_id);

create index if not exists instance_members_instance_id_completed_membership_id_index2
    on instance_members (instance_id, completed, membership_id);

create index if not exists instance_members_instance_id_index
    on instance_members (instance_id);

create index if not exists instance_members_membership_id_completed_index
    on instance_members (membership_id, completed);

create index if not exists instance_members_membership_id_index
    on instance_members (membership_id);

create index if not exists instance_members_platform_index
    on instance_members (platform);

create table if not exists instances
(
    id                     integer not null
        constraint instances_pk
            primary key autoincrement,
    instance_id            integer not null
        constraint instances_pk2
            unique,
    occurred_at            integer not null,
    starting_phase_index   integer not null,
    started_from_beginning integer not null,
    activity_hash          integer not null,
    activity_director_hash integer not null,
    is_private             integer not null,
    completed              integer not null,
    completion_reasons     TEXT    not null,
    created_at             integer not null,
    updated_at             integer not null,
    deleted_at             integer not null
)
    strict;

create index if not exists instances_activity_director_hash_index
    on instances (activity_director_hash);

create index if not exists instances_activity_hash_index
    on instances (activity_hash);

create index if not exists instances_competed_index
    on instances (completed);

create index if not exists instances_completion_reasons_index
    on instances (completion_reasons);

create index if not exists instances_completion_reasons_instance_id_index
    on instances (completion_reasons, instance_id);

create index if not exists instances_instance_id_competed_index
    on instances (instance_id, completed);

create index if not exists instances_instance_id_index
    on instances (instance_id);

create index if not exists instances_instance_id_started_from_beginning_index
    on instances (instance_id, started_from_beginning);

create index if not exists instances_is_private_index
    on instances (is_private);

create index if not exists instances_occurred_at_index
    on instances (occurred_at);

create index if not exists instances_started_from_beginning_index
    on instances (started_from_beginning);

create index if not exists instances_starting_phase_index_index
    on instances (starting_phase_index);

create table if not exists manifest_activities
(
    id                     integer not null
        constraint manifest_activities_pk
            primary key autoincrement,
    activity_type          integer not null,
    name                   TEXT    not null,
    description            TEXT    not null,
    hash                   integer not null
        constraint manifest_activities_pk2
            unique,
    "index"                integer not null,
    is_pvp                 integer not null,
    image_url              TEXT    not null,
    matchmaking_enabled    integer not null,
    fireteam_min_size      integer not null,
    fireteam_max_size      integer not null,
    max_players            integer not null,
    requires_guardian_oath integer not null,
    created_at             integer not null,
    updated_at             integer not null,
    deleted_at             integer not null
)
    strict;

create index if not exists manifest_activities_activity_type_index
    on manifest_activities (activity_type);

create index if not exists manifest_activities_hash_index
    on manifest_activities (hash);

create index if not exists manifest_activities_index_index
    on manifest_activities ("index");

create index if not exists manifest_activities_is_pvp_index
    on manifest_activities (is_pvp);

create index if not exists manifest_activities_matchmaking_enabled_index
    on manifest_activities (matchmaking_enabled);

create index if not exists manifest_activities_name_index
    on manifest_activities (name);

create table if not exists manifest_activity_types
(
    id          integer not null
        constraint manifest_activity_types_pk
            primary key autoincrement,
    hash        integer not null
        constraint manifest_activity_types_pk2
            unique,
    name        TEXT    not null,
    description TEXT    not null,
    icon_url    TEXT    not null,
    created_at  integer not null,
    updated_at  integer not null,
    deleted_at  integer not null,
    "index"     integer not null
)
    strict;

create table if not exists manifest_classes
(
    id         integer not null
        constraint manifest_classes_pk
            primary key autoincrement,
    hash       integer not null
        constraint manifest_classes_pk2
            unique,
    "index"    integer not null,
    type       integer not null,
    name       TEXT    not null,
    created_at integer not null,
    updated_at integer not null,
    deleted_at integer not null
)
    strict;

create index if not exists manifest_classes_index_index
    on manifest_classes ("index");

create index if not exists manifest_classes_type_index
    on manifest_classes (type);

create table if not exists manifest_seasons
(
    id         integer not null
        constraint manifest_seasons_pk
            primary key autoincrement,
    hash       integer not null
        constraint manifest_seasons_pk2
            unique,
    name       text    not null,
    pass_hash  integer not null,
    number     integer not null,
    starts_at  integer not null,
    ends_at    integer not null,
    created_at integer not null,
    updated_at integer not null,
    deleted_at integer not null
)
    strict;

create index if not exists manifest_seasons_hash_index
    on manifest_seasons (hash);

create index if not exists manifest_seasons_name_index
    on manifest_seasons (name);

create index if not exists manifest_seasons_number_index
    on manifest_seasons (number);

create table if not exists manifest_triumphs
(
    id          integer not null
        constraint manifest_triumphs_pk
            primary key autoincrement,
    hash        integer not null
        constraint manifest_triumphs_pk2
            unique,
    name        TEXT    not null,
    description TEXT    not null,
    title       TEXT    not null,
    is_title    integer not null,
    gilded      integer not null,
    created_at  integer not null,
    updated_at  integer not null,
    deleted_at  integer not null
)
    strict;

create index if not exists manifest_triumphs_gilded_index
    on manifest_triumphs (gilded);

create index if not exists manifest_triumphs_hash_index
    on manifest_triumphs (hash);

create index if not exists manifest_triumphs_is_title_index
    on manifest_triumphs (is_title);

create index if not exists manifest_triumphs_name_index
    on manifest_triumphs (name);

create index if not exists manifest_triumphs_title_index
    on manifest_triumphs (title);

create table if not exists member_activities
(
    id                     integer not null
        constraint member_activities_pk
            primary key autoincrement,
    membership_id          integer not null,
    character_id           integer not null,
    platform_played        integer not null,
    activity_hash          integer not null,
    activity_hash_director integer not null,
    instance_id            integer not null,
    mode                   integer not null,
    modes                  TEXT    not null,
    private                integer not null,
    occurred_at            integer not null,
    created_at             integer not null,
    updated_at             integer not null,
    deleted_at             integer not null,
    constraint member_activities_pk2
        unique (instance_id, membership_id, character_id)
)
    strict;

create index if not exists member_activities_activity_hash_director_index
    on member_activities (activity_hash_director);

create index if not exists member_activities_activity_hash_index
    on member_activities (activity_hash);

create index if not exists member_activities_character_id_index
    on member_activities (character_id);

create index if not exists member_activities_instance_id_index
    on member_activities (instance_id);

create index if not exists member_activities_membership_id_index
    on member_activities (membership_id);

create index if not exists member_activities_membership_id_instance_id_index
    on member_activities (membership_id, instance_id);

create index if not exists member_activities_mode_index
    on member_activities (mode);

create index if not exists member_activities_modes_index
    on member_activities (modes);

create index if not exists member_activities_occurred_at_index
    on member_activities (occurred_at);

create index if not exists member_activities_platform_played_index
    on member_activities (platform_played);

create index if not exists member_activities_private_index
    on member_activities (private);

create table if not exists member_activity_stats
(
    id            integer not null
        constraint member_activity_stats_pk
            primary key autoincrement,
    membership_id integer not null,
    character_id  integer not null,
    instance_id   integer not null,
    name          TEXT    not null,
    value         real    not null,
    value_display text    not null,
    created_at    integer not null,
    updated_at    integer not null,
    deleted_at    integer not null,
    constraint member_activity_stats_pk2
        unique (membership_id, character_id, instance_id, name)
)
    strict;

create index if not exists member_activity_stats_character_id_index
    on member_activity_stats (character_id);

create index if not exists member_activity_stats_instance_id_index
    on member_activity_stats (instance_id);

create index if not exists member_activity_stats_membership_id_index
    on member_activity_stats (membership_id);

create index if not exists member_activity_stats_membership_id_instance_id_index
    on member_activity_stats (membership_id, instance_id);

create index if not exists member_activity_stats_membership_id_instance_id_name_index
    on member_activity_stats (membership_id, instance_id, name);

create index if not exists member_activity_stats_name_index
    on member_activity_stats (name);

create index if not exists member_activity_stats_value_display_index
    on member_activity_stats (value_display);

create index if not exists member_activity_stats_value_index
    on member_activity_stats (value);

create table if not exists member_characters
(
    id                      integer not null
        constraint member_characters_pk
            primary key autoincrement,
    membership_id           integer not null,
    platform                integer not null,
    character_id            integer not null
        constraint member_characters_pk2
            unique,
    class_hash              integer not null,
    light                   integer not null,
    last_played_at          integer not null,
    minutes_played_session  integer not null,
    minutes_played_lifetime integer not null,
    emblem_hash             integer not null,
    emblem_url              text    not null,
    emblem_background_url   text    not null,
    created_at              integer not null,
    updated_at              integer not null,
    deleted_at              integer not null,
    constraint member_characters_pk3
        unique (platform, membership_id, character_id)
)
    strict;

create index if not exists member_characters_character_id_index
    on member_characters (character_id);

create index if not exists member_characters_class_hash_index
    on member_characters (class_hash);

create index if not exists member_characters_emblem_background_url_index
    on member_characters (emblem_background_url);

create index if not exists member_characters_emblem_hash_index
    on member_characters (emblem_hash);

create index if not exists member_characters_emblem_url_index
    on member_characters (emblem_url);

create index if not exists member_characters_membership_id_index
    on member_characters (membership_id);

create index if not exists member_characters_platform_index
    on member_characters (platform);

create table if not exists member_snapshots
(
    id            INTEGER not null
        constraint member_snapshots_pk
            primary key autoincrement,
    membership_id INTEGER not null,
    snapshot_name TEXT    not null,
    version       INTEGER not null,
    data          TEXT    not null,
    created_at    INTEGER not null,
    updated_at    INTEGER not null,
    deleted_at    INTEGER not null,
    constraint member_snapshots_pk2
        unique (snapshot_name, membership_id, version)
)
    strict;

create index if not exists member_snapshots_membership_id_index
    on member_snapshots (membership_id);

create index if not exists member_snapshots_snapshot_name_index
    on member_snapshots (snapshot_name);

create index if not exists member_snapshots_version_index
    on member_snapshots (version);

create table if not exists member_triumphs
(
    id              INTEGER not null
        constraint member_triumphs_pk
            primary key autoincrement,
    membership_id   INTEGER not null,
    hash            INTEGER not null,
    state           INTEGER not null,
    times_completed INTEGER not null,
    created_at      INTEGER not null,
    updated_at      INTEGER not null,
    deleted_at      INTEGER not null,
    constraint member_triumphs_pk2
        unique (membership_id, hash)
)
    strict;

create index if not exists member_triumphs_hash_index
    on member_triumphs (hash);

create index if not exists member_triumphs_membership_id_index
    on member_triumphs (membership_id);

create index if not exists member_triumphs_state_index
    on member_triumphs (state);

create index if not exists member_triumphs_times_completed_index
    on member_triumphs (times_completed);

create table if not exists members
(
    id                     INTEGER not null
        constraint members_pk
            primary key autoincrement,
    membership_id          INTEGER not null
        constraint members_pk2
            unique,
    platform               INTEGER not null,
    display_name           TEXT    not null,
    display_name_global    TEXT    not null,
    guardian_rank_current  INTEGER not null,
    guardian_rank_lifetime INTEGER not null,
    last_played_at         INTEGER not null,
    created_at             INTEGER not null,
    updated_at             INTEGER not null,
    deleted_at             INTEGER not null
)
    strict;

create index if not exists members_display_name_global_index
    on members (display_name_global);

create index if not exists members_display_name_index
    on members (display_name);

create index if not exists members_last_played_at_index
    on members (last_played_at);

create index if not exists members_platform_index
    on members (platform);

create table if not exists setting_modes
(
    id          integer not null
        constraint setting_modes_pk
            primary key autoincrement,
    leaderboard integer not null,
    dashboard   integer not null,
    name        text    not null
        constraint setting_modes_pk2
            unique,
    description text    not null,
    value       text    not null,
    "order"     integer not null,
    created_at  integer not null,
    updated_at  integer not null,
    deleted_at  integer not null
)
    strict;

create index if not exists setting_modes_dashboard_index
    on setting_modes (dashboard);

create index if not exists setting_modes_leaderboard_index
    on setting_modes (leaderboard);

create index if not exists setting_modes_order_index
    on setting_modes ("order");


