use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type StatsDb = Arc<Mutex<HashMap<String, Stat>>>;
pub type KeywordDb = Arc<Mutex<HashMap<String, Vec<MainLog>>>>;

pub fn blank_db() -> (StatsDb, KeywordDb) {
    // Todo: Loading for offline use
    // let aa = std::env::var("JSON_FILE_PATH");
    // let bb = std::env::var("ACCOUNT_NAME");

    // if aa.is_ok() && bb.is_ok() {
    //     let ff = std::fs::File::open(aa.unwrap()).unwrap();

    //     let json = serde_json::from_reader(ff).unwrap();

    //     let mut hm = HashMap::new();
    //     hm.insert(bb.unwrap(), json);

    //     return Arc::new(Mutex::new(hm));
    // }

    let stats_db = Arc::new(Mutex::new(HashMap::new()));
    let keywords_db = Arc::new(Mutex::new(HashMap::new()));

    (stats_db, keywords_db)
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Stat {
    pub error_counts: u64,
    pub running: bool,
    pub no_api_calls: u64,
    pub started_at: String,
    pub last_updated_at: String,
    pub logs: Vec<MainLog>,
    pub keywords: Vec<KeywordStat>,
}

impl Stat {
    pub fn new() -> Self {
        Stat {
            error_counts: 0,
            running: false,
            no_api_calls: 0,
            started_at: crate::helpers::current_time_string(),
            last_updated_at: crate::helpers::current_time_string(),
            logs: Vec::new(),
            keywords: Vec::new(),
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
pub struct MainLog {
    pub r#type: String,
    pub time: String,
    pub message: String,
    pub meta: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KeywordStat {
    pub id: u64,
    pub last_updated_at: String,
    pub name: Option<String>,
    pub keyword: Option<String>,
    pub placement: Option<u64>,
    pub running: Option<bool>,
    pub error_counts: Option<u64>,
    pub ads_running: Option<bool>,
    pub ads_position: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KeywordStatInput {
    pub id: u64,
    pub name: Option<String>,
    pub keyword: Option<String>,
    pub placement: Option<u64>,
    pub running: Option<bool>,
    pub error_counts: Option<u64>,
    pub ads_running: Option<bool>,
    pub ads_position: Option<u64>,
    pub logs: Option<Vec<MainLog>>,
}
