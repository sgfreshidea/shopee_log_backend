/// How our data look?
//  Main logs contains when bot started to run, what is total log amount
// Keywords logs contains indivitual keyword with their own logs
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;

use tokio::sync::RwLock;

type Account = String;
type KeywordId = u64;
type KeywordStats = HashMap<KeywordId, KeywordStatistics>;

#[derive(Debug)]
pub struct Statistics {
    pub main_stats: MainStats,
    pub keyword_stats: KeywordStats,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KeywordStatistics {
    pub stats: KeywordStat,
    pub keyword_logs: Vec<Log>,
}

impl Statistics {
    pub fn new(account: Account) -> Self {
        Statistics {
            main_stats: MainStats::new(account),
            keyword_stats: HashMap::new(),
        }
    }
}

pub type Db = Arc<RwLock<HashMap<Account, Statistics>>>;

pub fn blank_db() -> Db {
    Arc::new(RwLock::new(HashMap::new()))
}

// Stats is top level statistics
// It contains inner individual keyword statistics
// However every log related to keyword goes into keyword_db
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MainStats {
    // Name of Account
    pub account_name: String,

    // Total error counts. It is keyword error + other errors
    pub error_counts: u64,

    // Total number of logs . It is main log + all log from keywords
    pub log_counts: u64,

    // Currently unused
    pub running: bool,

    // Total api calls made by other bot .
    pub no_api_calls: u64,

    // When the other bot was started?
    pub started_at: String,

    // when the bot was last updated. When new logs , keywoord logs come this field must be updated
    pub last_updated_at: String,

    // Logs are cleared out and only top 100 logs are placed if program memory goes beyond
    // 1G
    pub logs: Vec<Log>,
}

impl MainStats {
    pub fn new(account_name: Account) -> Self {
        MainStats {
            account_name,
            error_counts: 0,
            running: false,
            no_api_calls: 0,
            log_counts: 0,
            started_at: crate::helpers::current_time_string(),
            last_updated_at: crate::helpers::current_time_string(),
            logs: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateStat {
    pub error_counts: Option<u64>,
    pub running: Option<bool>,

    // How many api calls were made since last updated
    pub no_of_api_call_diff: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Log {
    pub r#type: String,
    pub time: String,
    pub message: String,
    pub meta: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KeywordStat {
    pub id: u64,
    pub last_updated_at: String,
    pub error_counts: u64,
    pub log_counts: u64,

    pub name: Option<String>,
    pub keyword: Option<String>,
    pub placement: Option<u64>,
    pub running: Option<bool>,
    pub ads_running: Option<bool>,
    pub ads_position: Option<u64>,
    pub current_price: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateKeywordStat {
    pub id: u64,
    pub name: Option<String>,
    pub current_price: Option<f64>,
    pub keyword: Option<String>,
    pub placement: Option<u64>,
    pub running: Option<bool>,
    pub error_counts: Option<u64>,
    pub ads_running: Option<bool>,
    pub ads_position: Option<u64>,
    pub logs: Option<Vec<Log>>,
}

impl KeywordStatistics {
    pub fn update(stats: &mut Statistics, input: &UpdateKeywordStat) {
        let main_stats = &mut stats.main_stats;
        main_stats.last_updated_at = crate::helpers::current_time_string();

        let keyword_stats = &mut stats.keyword_stats;

        if let Some(ks) = keyword_stats.get_mut(&input.id) {
            ks.stats.last_updated_at = crate::helpers::current_time_string();

            if let Some(ru) = input.running {
                ks.stats.running = Some(ru);
            }

            // Todo use if let some feeling lazy atm.
            if input.ads_running.is_some() {
                ks.stats.ads_running = input.ads_running
            }

            if input.ads_position.is_some() {
                ks.stats.ads_position = input.ads_position
            }

            if let Some(cp) = input.current_price {
                ks.stats.current_price = Some(cp);
            }
        } else {
            let keyword_statistics = KeywordStatistics {
                stats: KeywordStat {
                    id: input.id,
                    error_counts: 0,
                    log_counts: 0,
                    name: input.name.to_owned(),
                    keyword: input.keyword.to_owned(),
                    placement: input.placement,
                    last_updated_at: crate::helpers::current_time_string(),
                    running: input.running,
                    ads_running: input.ads_running,
                    ads_position: input.ads_position,
                    current_price: input.current_price,
                },
                keyword_logs: Vec::with_capacity(1000),
            };

            keyword_stats.insert(input.id, keyword_statistics);
        }
    }

    pub fn add_logs(stats: &mut Statistics, id: KeywordId, input: Log) {
        let main_stats = &mut stats.main_stats;

        let keyword_stats = &mut stats.keyword_stats;
        if let Some(ks) = keyword_stats.get_mut(&id) {
            main_stats.last_updated_at = crate::helpers::current_time_string();

            if input.r#type == "error" {
                main_stats.error_counts += 1;
            }

            main_stats.log_counts += 1;

            ks.keyword_logs.push(input);
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct BackupStatistics {
    stats: MainStats,
    keyword: HashMap<KeywordId, Vec<Log>>,
}

// // We might want to reanalyze previous record for that we are providing ability to
// // use old database.
// pub async fn load_old_database() -> Option<Statistics> {
//     let aa = std::env::var("JSON_FILE_PATH");

//     if aa.is_ok() {
//         let ff = std::fs::File::open(aa.unwrap()).unwrap();

//         let json: BackupStatistics = serde_json::from_reader(ff).unwrap();

//         let stats = json.stats;
//         let keyword = json.keyword;

//         let account = stats.account_name.to_owned();

//         let mut stats_hm = HashMap::new();
//         stats_hm.insert(account.clone(), stats);

//         let mut keywords_hm = HashMap::new();
//         keywords_hm.insert(account, keyword);

//         let arc_stats = Arc::new(Mutex::new(stats_hm));
//         let arc_keywords = Arc::new(Mutex::new(keywords_hm));

//         return Some(Statistics {
//             stats: arc_stats,
//             keyword_stats: arc_keywords,
//         });
//     }

//     None
// }

// pub async fn clear_database_after_getting_more_than_1GB(db: Db) {
//     // As database keeeps growing we must keep memory usage in track
//     // For this  we will check how much process is using memory
//     // if its greator than zero we will clear it

//     let mut statistics = *db;
//     for st in statistics.as_mut() {
//         clear_db(&mut st).await;
//     }
// }

// pub async fn clear_db(statistics: &mut Statistics) {
//     use std::borrow::Cow;

//     #[derive(Debug, Deserialize, Serialize, Clone)]
//     struct Backup<'a> {
//         stats: Cow<'a, Stat>,
//         keyword: Cow<'a, [Log]>,
//     };

//     let mut lock = db.lock().await;
//     let stats = lock.entry(account.clone()).or_insert(Stat::new());

//     let mut klock = keyword_db.lock().await;
//     let kstats = klock.entry(account.clone()).or_insert(Vec::new());

//     // {
//     //     let content = serde_json::to_string_pretty(&Backup {
//     //         stats: Cow::Borrowed(&*stats),
//     //         keyword: Cow::Borrowed(&*kstats),
//     //     })
//     //     .unwrap();

//     //     let path = crate::helpers::sanitize(&crate::helpers::current_time_string()) + ".json";

//     //     let mut new_file = File::create(path).unwrap();
//     //     new_file.write_all(&content.into_bytes()).unwrap();
//     // }

//     let current_stats_len = (*stats).logs.len();

//     // [1,2,3,4,5,6,7] to keep 2 elem drain 0..7-2
//     if current_stats_len > 100 {
//         (*stats).logs.drain(0..(current_stats_len - 100));
//     }
// }
