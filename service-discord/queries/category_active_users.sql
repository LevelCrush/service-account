WITH
target_messages AS (
SELECT
    channel_logs.member_id,
    channel_logs.message_timestamp AS message_timestamp
FROM channel_logs
WHERE channel_logs.category_name = ?
AND channel_logs.message_timestamp > ?
)
SELECT
    target_messages.member_id,
    COALESCE(MAX(target_messages.message_timestamp), 0) AS message_timestamp
FROM target_messages
GROUP BY target_messages.member_id