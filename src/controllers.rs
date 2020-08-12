use crate::models::{Db, KeywordStat, MainLog, Stat, UpdateStat};
use serde_json::json;
use std::convert::Infallible;

pub async fn list_accounts(db: Db) -> Result<impl warp::Reply, Infallible> {
    let lock = db.lock().await;

    let keys = lock.keys().into_iter().collect::<Vec<&String>>();

    let json = serde_json::json!({ "accounts": keys });
    Ok(warp::reply::json(&json))
}

pub async fn stats(account: String, db: Db) -> Result<impl warp::Reply, Infallible> {
    let mut lock = db.lock().await;
    let stats = lock.entry(account).or_insert(Stat::new());

    Ok(warp::reply::json(&*stats))
}

pub async fn clear_log(account: String, db: Db) -> Result<impl warp::Reply, Infallible> {
    let mut lock = db.lock().await;
    let stats = lock.entry(account).or_insert(Stat::new());

    *stats = Stat::new();

    Ok(warp::reply::json(&*stats))
}

pub async fn update_stats(
    account: String,
    ss: UpdateStat,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let mut lock = db.lock().await;
    let stats = lock.entry(account).or_insert(Stat::new());

    if ss.error_counts.is_some() {
        stats.error_counts += ss.error_counts.unwrap();
    }

    if ss.running.is_some() {
        stats.running = ss.running.unwrap();
    }

    Ok(warp::reply::json(&json!({
        "type": "success",
        "data":{"stat": stats.clone()}
    })))
}

pub async fn add_logs_to_stats(
    account: String,
    ss: MainLog,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let mut lock = db.lock().await;
    let stats = lock.entry(account).or_insert(Stat::new());

    if ss.r#type == "error" {
        stats.error_counts += 1
    }

    stats.logs.push(ss);

    Ok(warp::reply::json(&json!({
        "type": "success",
        "data":{"stat": stats.clone()}
    })))
}

pub async fn set_keywords_to_stats(
    account: String,
    ss: Vec<KeywordStat>,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let mut lock = db.lock().await;
    let stats = lock.entry(account).or_insert(Stat::new());

    stats.keywords = ss;

    Ok(warp::reply::json(&json!({
        "type": "success",
        "data":{"stat": stats.clone()}
    })))
}

pub async fn add_logs_to_keyword(
    account: String,
    id: u64,
    ss: MainLog,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let mut lock = db.lock().await;
    let stats = lock.entry(account).or_insert(Stat::new());

    if ss.r#type == "error" {
        stats.error_counts += 1;
    }

    for keyword in stats.keywords.iter_mut() {
        if keyword.id == id {
            if keyword.logs.is_none() {
                keyword.logs = Some(vec![ss.clone()]);
            } else {
                keyword.logs.as_mut().map(|v| {
                    v.push(ss.clone());
                });
            }

            if ss.r#type == "error" {
                let current_count = keyword.error_counts.as_ref().unwrap_or(&0);

                keyword.error_counts = Some(*current_count);
            }
        }
    }

    Ok(warp::reply::json(&json!({
        "type": "success",
        "data":{"stat": stats.clone()}
    })))
}

pub async fn update_keyword_stat(
    account: String,
    ss: KeywordStat,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let mut lock = db.lock().await;
    let stats = lock.entry(account).or_insert(Stat::new());

    for keyword in stats.keywords.iter_mut() {
        if keyword.id == ss.id {
            if ss.running.is_some() {
                keyword.running = ss.running
            }

            if ss.ads_running.is_some() {
                keyword.ads_running = ss.ads_running
            }
        }
    }

    Ok(warp::reply::json(&json!({
        "type": "success",
        "data":{"stat": stats.clone()}
    })))
}
