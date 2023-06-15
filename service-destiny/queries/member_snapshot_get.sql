SELECT *
FROM member_snapshots 
WHERE member_snapshots.membership_id = ?
AND member_snapshots.snapshot_name = ?
AND member_snapshots.version = ?