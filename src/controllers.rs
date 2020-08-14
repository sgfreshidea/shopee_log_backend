use crate::models::{Db, KeywordStat, MainLog, Stat, UpdateStat};
use serde_json::json;
use std::convert::Infallible;
use std::fs::File;
use std::io::prelude::*;

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

    // Micro optimization is possible here.

    let content = serde_json::to_string_pretty(&mut *stats).unwrap();

    let mut new_file = File::create(crate::helpers::current_time_string()).unwrap();
    new_file.write_all(&content.into_bytes()).unwrap();

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
    stats.last_updated_at = crate::helpers::current_time_string();

    if ss.error_counts.is_some() {
        stats.error_counts += ss.error_counts.unwrap();
    }

    if ss.running.is_some() {
        stats.running = ss.running.unwrap();
    }

    if ss.no_of_api_call_diff.is_some() {
        stats.no_api_calls += ss.no_of_api_call_diff.unwrap();
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

    stats.last_updated_at = crate::helpers::current_time_string();

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

    stats.last_updated_at = crate::helpers::current_time_string();

    for ii in ss.iter() {
        let mut pushed = false;

        for mut jj in stats.keywords.iter_mut() {
            if ii.id == jj.id {
                pushed = true;
                jj.running = ii.running;
                jj.logs
                    .as_mut()
                    .map(|v| v.extend_from_slice(&ii.logs.as_ref().unwrap_or(&vec![])));
            }
        }

        if pushed == false {
            stats.keywords.push(ii.clone());
        }
    }

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

    stats.last_updated_at = crate::helpers::current_time_string();

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

            keyword.last_updated_at = Some(crate::helpers::current_time_string());

            if ss.r#type == "error" {
                let current_count = keyword.error_counts.as_ref().unwrap_or(&0);

                keyword.error_counts = Some(*current_count + 1);
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

    stats.last_updated_at = crate::helpers::current_time_string();

    for keyword in stats.keywords.iter_mut() {
        if keyword.id == ss.id {
            keyword.last_updated_at = Some(crate::helpers::current_time_string());

            if ss.running.is_some() {
                keyword.running = ss.running
            }

            if ss.ads_running.is_some() {
                keyword.ads_running = ss.ads_running
            }

            if ss.ads_position.is_some() {
                keyword.ads_position = ss.ads_position
            }
        }
    }

    Ok(warp::reply::json(&json!({
        "type": "success",
        "data":{"stat": stats.clone()}
    })))
}
