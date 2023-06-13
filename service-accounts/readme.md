# Service: Accounts

This service is intended to provide a login service for members to use. The end goal is to be able to link all bungie accounts/twitch/etc to their Discord account. This service is **session** enabled.

All logins are done through OAuth and the target platforms official third party login service.

No sensitive information is ever stored here, nor does the service care about it.

The service just wants the user ids/minimal amount of information required to to link our users together.

Below is a list of routes that one could possibly query or land on.

Format is described inbetween the next two lines

#

Effect: Route description/what it does/effects

```
HTTP METHOD: Route here
```

#

## Basic Routes

Effect: Redirects to `/platform/discord/login`

```
GET: /login
```

Effect: Destroy's the user session and logs the user out.

```
GET: /logout
```

#

## Discord Routes

Effect: Redirects to Discord OAuth

```
GET: /platform/discord/login
```

Effect: Validates response that comes back from discord to make sure we have a legit response.

```
GET: /platform/discord/validate
```

#

## Twitch Routes

Effect: Redirects to Twitch OAuth

```
GET: /platform/twitch/login
```

Effect: Validates response that comes back from twitch to make sure we have a legit response.

```
GET: /platform/twitch/validate
```

Effect: Unlinks the twitch account from the user that is logged in

```
GET: /platform/twitch/unlink
```

#

## Bungie Routes

Effect: Redirects to Bungie OAuth

```
GET: /platform/bungie/login
```

Effect: Validates response that comes back from bungie to make sure we have a legit response.

```
GET: /platform/bungie/validate
```

Effect: Unlinks the bungie account from the user that is logged in

```
GET: /platform/bungie/unlink
```

#

## Profile Route

Effect: Gets a JSON view of the logged in user state

```
GET: /profile/json
```

## Search Routes

Effect: Search for a Discord user by supplying their bungie id.

```
GET: /search/by/bungie/:bungie_name
```

Effect: Search for a mass amount of discord user by supplying their bungie ids. The body sent in POST must be a JSON type and is expecting an array of strings.

```
POST: /search/by/bungie
```
