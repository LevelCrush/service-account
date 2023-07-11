use super::report::member::MemberReport;
use crate::database::activity_history::NetworkBreakdownResult;
use crate::database::clan::ClanInfoResult;
use crate::database::instance::InstanceMemberRecord;
use crate::database::leaderboard::LeaderboardEntryResult;
use crate::database::member::MemberResult;
use crate::database::seasons::SeasonRecord;
use crate::database::setting::SettingModeRecord;
use crate::database::triumph::TriumphTitleResult;
use crate::{bungie::BungieClient, database::activity_history::ActivityHistoryRecord};
use levelcrush::retry_lock::RetryLock;
use levelcrush::task_manager::TaskManager;
use levelcrush::{cache::MemoryCache, database};
use sqlx::MySqlPool;
use std::boxed::Box;
use std::collections::HashMap;
use std::time::Duration;

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
    pub database: MySqlPool,            // MySqlPool is already wrapped in a arc, safe to clone
    pub bungie: BungieClient,           // safe to clone, underlying implementation uses handles/arc
    pub cache: MemoryCache<CacheItem>,  // memory cache uses Arc's internally. Safe to clone
    pub task_running: MemoryCache<u64>, // keep track whenever we started these task, at the moment only used by member reports
    pub settings: MemoryCache<Setting>,
    pub leaderboards: MemoryCache<Vec<LeaderboardEntryResult>>,
    pub ranks: MemoryCache<Vec<LeaderboardEntryResult>>,
    pub seasons: MemoryCache<Vec<SeasonRecord>>,
    pub tasks: TaskManager,          // also used by member reports
    pub priority_tasks: TaskManager, // also used by member reports
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

        AppState {
            database,
            seasons: MemoryCache::new(),
            ranks: MemoryCache::new(),
            leaderboards: MemoryCache::new(),
            settings: MemoryCache::new(),
            bungie: BungieClient::new(),
            cache: MemoryCache::new(), // cache for 24 hours (a members profile does not update this often, except for last login at and character)      }
            task_running: MemoryCache::new(),
            tasks: TaskManager::new(max_task_workers),
            priority_tasks: TaskManager::new(priority_task_workers),
            locks: RetryLock::new(10, Duration::from_secs(60)),
        }
    }
}
