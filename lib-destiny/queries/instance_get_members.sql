SELECT
    instance_members.id,
    instance_members.membership_id,
    instance_members.character_id
FROM instance_members   
WHERE instance_members.instance_id = ?