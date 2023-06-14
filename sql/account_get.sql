SELECT
    accounts.*
FROM `levelcrush_accounts`.accounts AS accounts
WHERE accounts.token = ?
AND accounts.token_secret = ?
LIMIT 1