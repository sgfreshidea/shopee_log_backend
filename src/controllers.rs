use crate::models::{Db, KeywordStatistics, Log, Statistics, UpdateKeywordStat, UpdateStat};
use serde_json::json;
use serde_json::Value::Null;
use std::convert::Infallible;
use warp::reply::json;

pub async fn list_accounts(db: Db) -> Result<impl warp::Reply, Infallible> {
    let lock = db.read().await;

    let keys = lock.keys().into_iter().collect::<Vec<&String>>();
    let json = serde_json::json!({ "accounts": keys });
    Ok(warp::reply::json(&json))
}

pub async fn get_keyword_logs(
    account: String,
    keyword_id: u64,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let lock = db.read().await;

    if let Some(statistics) = lock.get(&account) {
        let keyword_stats = &statistics.keyword_stats;

        if let Some(keyword_logs) = keyword_stats.get(&keyword_id) {
            return Ok(json(keyword_logs));
        }
    }

    Ok(json(&Null))
}

pub async fn get_main_stats(account: String, db: Db) -> Result<impl warp::Reply, Infallible> {
    let lock = db.read().await;
    if let Some(statistics) = lock.get(&account) {
        let main_stats = &statistics.main_stats;
        return Ok(json(main_stats));
    }

    Ok(json(&Null))
}

pub async fn clear_log(account: String, db: Db) -> Result<impl warp::Reply, Infallible> {
    // For now just send success message
    Ok(json(&json!({"type": "success",})))
}

pub async fn update_stats(
    account: String,
    req: UpdateStat,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let mut lock = db.write().await;

    if let Some(statistics) = lock.get_mut(&account) {
        let mut main_stats = &mut statistics.main_stats;
        main_stats.last_updated_at = crate::helpers::current_time_string();

        if req.error_counts.is_some() {
            main_stats.error_counts += req.error_counts.unwrap();
        }

        if req.running.is_some() {
            main_stats.running = req.running.unwrap();
        }

        if req.no_of_api_call_diff.is_some() {
            main_stats.no_api_calls += req.no_of_api_call_diff.unwrap();
        }
    } else {
        lock.insert(account.clone(), Statistics::new(account));
    }

    Ok(json(&json!({
        "type": "success",
    })))
}

pub async fn add_logs_to_stats(
    account: String,
    req: Log,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let mut lock = db.write().await;

    if let Some(stats) = lock.get_mut(&account) {
        let main_stats = &mut stats.main_stats;
        main_stats.last_updated_at = crate::helpers::current_time_string();

        if req.r#type == "error" {
            main_stats.error_counts += 1
        }

        main_stats.logs.push(req);
    }

    Ok(json(&json!({"type": "success",})))
}

pub async fn set_keywords_to_stats(
    account: String,
    input: Vec<UpdateKeywordStat>,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let mut lock = db.write().await;

    if let Some(stats) = lock.get_mut(&account) {
        let main_stats = &mut stats.main_stats;

        main_stats.last_updated_at = crate::helpers::current_time_string();

        for ii in input.iter() {
            KeywordStatistics::update(stats, ii)
        }
    }

    Ok(json(&json!({"type": "success",})))
}

pub async fn add_logs_to_keyword(
    account: String,
    id: u64,
    input: Log,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let mut lock = db.write().await;

    if let Some(stats) = lock.get_mut(&account) {
        KeywordStatistics::add_logs(stats, id, input)
    }

    Ok(json(&json!({"type": "success",})))
}

pub async fn update_keyword_stat(
    account: String,
    input: UpdateKeywordStat,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let mut lock = db.write().await;
    let stats = lock.get_mut(&account);

    if let Some(stats) = stats {
        KeywordStatistics::update(stats, &input);
    }

    Ok(json(&json!({"type": "success"})))
}
