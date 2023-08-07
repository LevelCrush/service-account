INSERT INTO member_characters
(
    membership_id, 
    platform,
    character_id, 
    class_hash, 
    light, 
    last_played_at,
    minutes_played_session,
    minutes_played_lifetime,
    emblem_hash,
    emblem_url,
    emblem_background_url,
    created_at,
    updated_at,
    deleted_at
)
VALUES(?,?,?,?,?,?,?,?,?,?,?,?,0,0)