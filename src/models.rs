use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type Db = Arc<Mutex<HashMap<String, Stat>>>;

pub fn blank_db() -> Db {
    Arc::new(Mutex::new(HashMap::new()))
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Stat {
    pub error_counts: u64,
    pub running: bool,
    pub logs: Vec<MainLog>,
    pub keywords: Vec<KeywordStat>,
}

impl Stat {
    pub fn new() -> Self {
        Stat {
            error_counts: 0,
            running: false,
            logs: Vec::new(),
            keywords: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateStat {
    pub error_counts: Option<u64>,
    pub running: Option<bool>,
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
    pub keyword: Option<String>,
    pub placement: Option<u64>,
    pub running: Option<bool>,
    pub error_counts: Option<u64>,
    pub ads_running: Option<bool>,
    pub ads_position: Option<u64>,
    pub logs: Option<Vec<MainLog>>,
}
