# Service: Automations

Automations is intended to **only** handle functions that can be callable via the web.

All routes are protected by a Public+Private key that is set by the server env.

These .env variables are

```bash
KEY_AUTOMATION_ACCESS_PUBLIC = "placeholder2"
KEY_AUTOMATION_ACCESS_PRIVATE = "placeholder3"
```

The incoming GET and POST request **MUST** have these two fields present in the headers as

PUBLIC-KEY, PRIVATE-KEY respectively.
