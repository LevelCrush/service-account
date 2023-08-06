use std::boxed::Box;
use std::collections::HashMap;
use std::time::Duration;

use levelcrush::alias::UnixTimestamp;
use sqlx::SqlitePool;

use levelcrush::retry_lock::RetryLock;
use levelcrush::task_pool::TaskPool;
use levelcrush::{cache::MemoryCache, database};

use crate::database::activity_history::NetworkBreakdownResult;
use crate::database::clan::ClanInfoResult;
use crate::database::instance::InstanceMemberRecord;
use crate::database::leaderboard::LeaderboardEntryResult;
use crate::database::member::MemberResult;
use crate::database::seasons::SeasonRecord;
use crate::database::setting::SettingModeRecord;
use crate::database::triumph::TriumphTitleResult;
use crate::env::AppVariable;
use crate::{bungie::BungieClient, database::activity_history::ActivityHistoryRecord, env};

use super::report::member::MemberReport;

#[derive(Clone, Debug)]
pub enum CacheItem {
    Member(Box<MemberResult>),
    MemberArray(Vec<MemberResult>),
    MemberSearchCount(u32),
    ClanInfo(Box<ClanInfoResult>),
    ClanInfoArray(Vec<ClanInfoResult>),
    ActivityHistoryArray(Vec<ActivityHistoryRecord>),
    InstanceMemberArray(Vec<InstanceMemberRecord>),
    MemberReport(Box<MemberReport>),
    MemberTitles(Vec<TriumphTitleResult>),
    NetworkBreakdown(HashMap<i64, NetworkBreakdownResult>),
}

#[derive(Clone, Debug)]
pub enum Setting {
    Modes(Vec<SettingModeRecord>),
}

#[derive(Clone)]
pub struct AppState {
    pub database: SqlitePool,
    // SqlitePool is already wrapped in a arc, safe to clone
    pub bungie: BungieClient,
    // safe to clone, underlying implementation uses handles/arc
    pub cache: MemoryCache<CacheItem>,
    // memory cache uses Arc's internally. Safe to clone
    pub task_running: MemoryCache<UnixTimestamp>,
    // keep track whenever we started these task, at the moment only used by member reports
    pub settings: MemoryCache<Setting>,
    pub leaderboards: MemoryCache<Vec<LeaderboardEntryResult>>,
    pub ranks: MemoryCache<Vec<LeaderboardEntryResult>>,
    pub seasons: MemoryCache<Vec<SeasonRecord>>,
    pub tasks: TaskPool,
    // also used by member reports
    pub priority_tasks: TaskPool,
    // also used by member reports
    pub locks: RetryLock,
}

impl AppState {
    /// Construct an app state
    ///
    /// Note: This will create a new database pool as well as a new bungie client
    pub async fn new() -> AppState {
        let database = database::connect().await;

        let max_task_workers = std::env::var("TASK_WORKERS")
            .unwrap_or_default()
            .parse::<usize>()
            .unwrap_or(1);

        let priority_task_workers = std::env::var("PRIORITY_TASK_WORKERS")
            .unwrap_or_default()
            .parse::<usize>()
            .unwrap_or(2);

        let bungie_api_key = env::get(AppVariable::BungieAPIKey);

        AppState {
            database,
            seasons: MemoryCache::new(),
            ranks: MemoryCache::new(),
            leaderboards: MemoryCache::new(),
            settings: MemoryCache::new(),
            bungie: BungieClient::new(bungie_api_key),
            cache: MemoryCache::new(), // cache for 24 hours (a members profile does not update this often, except for last login at and character)      }
            task_running: MemoryCache::new(),
            tasks: TaskPool::new(max_task_workers),
            priority_tasks: TaskPool::new(priority_task_workers),
            locks: RetryLock::new(10, Duration::from_secs(60)),
        }
    }
}
