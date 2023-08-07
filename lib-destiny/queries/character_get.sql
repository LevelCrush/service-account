SELECT
    member_characters.*
FROM member_characters
WHERE member_characters.character_id = ?
LIMIT 1