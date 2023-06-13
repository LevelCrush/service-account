# Level Crush

This is a mono repo intended to host all level crush related projects.

The following services can be found here

- [Website](https://github.com/LevelCrush/levelcrush/tree/main/website)
- [Service-Destiny](https://github.com/LevelCrush/levelcrush/tree/main/service-destiny)
- [Service-Accounts](https://github.com/LevelCrush/levelcrush/tree/main/service-accounts)
- [Service-Feed](https://github.com/LevelCrush/levelcrush/tree/main/service-feed)
- [Service-Automations](https://github.com/LevelCrush/levelcrush/tree/main/service-automations)
- [Service-Assets](https://github.com/LevelCrush/levelcrush/tree/main/service-assets)

There are an additional three libraries defined here that are also stored and used between the above services.

- [Lib-LevelCrush-RS](https://github.com/LevelCrush/levelcrush/tree/main/lib-levelcrush-rs)
- [Lib-LevelCrush-Macros-RS](https://github.com/LevelCrush/levelcrush/tree/main/lib-levelcrush-macros-rs)
- [Lib-LevelCrush-TS](https://github.com/LevelCrush/levelcrush/tree/main/lib-levelcrush-ts)

The libraries themselves have the language they are intended to be used with appended at the end. So

- [Lib-LevelCrush-RS](https://github.com/LevelCrush/levelcrush/tree/main/lib-levelcrush-rs) is intended to be used with Rust applications
- [Lib-LevelCrush-Macros-RS](https://github.com/LevelCrush/levelcrush/tree/main/lib-levelcrush-macros-rs) is intended to be used with Rust applications
- [Lib-LevelCrush-TS](https://github.com/LevelCrush/levelcrush/tree/main/lib-levelcrush-ts) is intended to be used with Typescript related projects.

# Lib-Levelcrush-RS

The project specific readme can be found [here](https://github.com/LevelCrush/levelcrush/tree/main/lib-levelcrush-rs). A brief overview of what this library is as follows.

### Feature Flags

The following feature flags can be set

- `server`: the crate will provide a **axum** based server for you to use
- `database`: the crate will provide database support via **sqlx** and sets up a connection pool for you
- `cors`: the crate will provide the ability to have a Cors layer (preconfigured to whatever host we need it to)
- `session`: the crate will provide a in memory session layer. If `database` is specified as well. Then it will provide a database version as well. You can choose between the two layers if you want.
- `default`: all above features are enabled.

### Async/Await

The library provides support for async/await via [Tokio](https://tokio.rs/). Seemed like a natural fit considered the use of axum, and the community documentation around it.

### Web Server

The library provides a implementation to quickly spin up a [Axum](https://github.com/tokio-rs/axum) based web server. There are some brief utilities included with it. Enabled by setting the crate feature `server` when included. The implementation allows you to turn on _rate limiting_ and _cors_ layer regardless of feature set. By turning on the `session` feature flag, you can opt into a in memory based session for the server automatically. Likewise By also enabling the `database` flag, you will have the opportunity to turn on the mysql session layer.

### Database Pool

The library will help spin up a MySqlPool based on supplied settings from a .env file located relative to the project. I opted to go with [sqlx](https://github.com/launchbadge/sqlx) for this over a ORM.

### Utility

The library also provides some utility functioinality and implementations. Utility includes

- `unix_timestamp()`: Uses `chrono` crate to generate a unix timestamp. [View Code](https://github.com/LevelCrush/levelcrush/blob/main/lib-levelcrush-rs/src/util.rs)
- `MemoryCache<T>`: Thread safe + Async supported way to have a persistent in memory cache. Can work across multiple threads if neccessary with read/write support and provides a way to attach a `Duration` to the cached value to support expiration/pruning. [View Code](https://github.com/LevelCrush/levelcrush/blob/main/lib-levelcrush-rs/src/cache.rs)
- `TaskManager`: Implements task pooling by using Tokio's `JoinHandle<T>`. This can be configured to run a certain amount of task at once (created by spawning a task with `tokio::spawn`). `TaskManager` is thread safe. At the moment, for sake of simplicity, `TaskManager` does not store the results of the task that are run. These are meant to be indepdendent long running functions that should execute at the same time, that end using some other medium to store / fetch data.
  An example in use can be found in the `service-destiny` app, when we are requesting a member report. The member report can take a very long time, rather then holding up the web request, a task is thrown onto the `TaskManager` and then handled in the background. The actual results are stored in the respective `MemoryCache<T>` to fetch the results when the member report is checked again.
  [View Code](https://github.com/LevelCrush/levelcrush/blob/main/lib-levelcrush-rs/src/task_manager.rs)
- `RetryLock`: Similiar to a mutex, except the application decides when to unlock/lock it. Good for request that tend to **write** data to the database/make external api calls and you don't want to flood the app with too many causing a deadlock. An example can be found by visiting this [link](https://github.com/LevelCrush/levelcrush/blob/main/service-accounts/src/routes/profile.rs) where it is used by the `service-accounts` app and it locks the user request from multiple profile request at once (Until cached). Since the `/profile/json` route is intended to be hit **many** times, it is possible for the same user to request and have us fetching/writing the same data numerous times causing too many database queries to go out/writing to the session, which can cause deadlocks. This utility limits those request and only stalls it for the user tied to the session. So other users are not held back. In the event of a `RetryLock` stalling too long, the lock is automatically released letting the operation to take place as a precaution. This duration/retry amount is configurable. [View Code](https://github.com/LevelCrush/levelcrush/blob/main/lib-levelcrush-rs/src/retry_lock.rs)

# Lib-LevelCrush-Macros-Rs

Macros used by all the services to help reduce boilerplate and provide common derive functions.

# Lib-LevelCrush-TS

Typescript library that exports bindings from the Lib-LevelCrush-RS and Service applications. These bindings are generated automatically by running the following command in the workspace OR the application directory.

```
cargo test
```

Once the bindings have been generated, they can be compiled to JS for js/ts consumption by running the following command.

```
npx tsc --build
```

There is a [tsconfig.lib.json](https://github.com/LevelCrush/levelcrush/blob/main/tsconfig.lib.json) at the root of the workspace that helps setup the required path for use in other typescript based projects.
