use crate::models::{KeywordDb, KeywordStat, KeywordStatInput, MainLog, Stat, StatsDb, UpdateStat};
use serde_json::json;
use std::convert::Infallible;
use std::fs::File;
use std::io::prelude::*;

pub async fn list_accounts(db: StatsDb) -> Result<impl warp::Reply, Infallible> {
    let lock = db.lock().await;

    let keys = lock.keys().into_iter().collect::<Vec<&String>>();

    let json = serde_json::json!({ "accounts": keys });
    Ok(warp::reply::json(&json))
}

pub async fn get_keyword_logs(
    account: String,
    keyword_id: u64,
    db: KeywordDb,
) -> Result<impl warp::Reply, Infallible> {
    let key = format!("{}{}", account, keyword_id);

    let mut lock = db.lock().await;
    let stats = lock.entry(key).or_insert(Vec::new());
    Ok(warp::reply::json(&*stats))
}

pub async fn stats(account: String, db: StatsDb) -> Result<impl warp::Reply, Infallible> {
    let mut lock = db.lock().await;
    let stats = lock.entry(account).or_insert(Stat::new());

    Ok(warp::reply::json(&*stats))
}

pub async fn clear_log(account: String, db: StatsDb) -> Result<impl warp::Reply, Infallible> {
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
    db: StatsDb,
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
    db: StatsDb,
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
    input: Vec<KeywordStatInput>,
    db: StatsDb,
    keyword_db: KeywordDb,
) -> Result<impl warp::Reply, Infallible> {
    let mut dbl = db.lock().await;
    let stats = dbl.entry(account.clone()).or_insert(Stat::new());

    stats.last_updated_at = crate::helpers::current_time_string();

    for ii in input.iter() {
        let mut pushed = false;

        for mut jj in stats.keywords.iter_mut() {
            if ii.id == jj.id {
                pushed = true;
                jj.running = ii.running;

                let mut kwl = keyword_db.lock().await;
                let keyword_hm_key = format!("{}{}", account, jj.id);
                let keyword_logs = kwl.entry(keyword_hm_key).or_insert(Vec::new());

                if ii.logs.is_none() {
                    let slice = ii.logs.as_ref().unwrap();
                    keyword_logs.extend_from_slice(&slice);
                }
            }
        }

        if pushed == false {
            stats.keywords.push(KeywordStat {
                id: ii.id,
                last_updated_at: crate::helpers::current_time_string(),
                name: ii.name,
                keyword: ii.keyword,
                placement: ii.placement,
                running: ii.running,
                error_counts: ii.error_counts,
                ads_running: ii.ads_running,
                ads_position: ii.ads_position,
            });
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
    db: StatsDb,
    keyword_db: KeywordDb,
) -> Result<impl warp::Reply, Infallible> {
    let mut lock = db.lock().await;
    let stats = lock.entry(account.clone()).or_insert(Stat::new());

    stats.last_updated_at = crate::helpers::current_time_string();

    if ss.r#type == "error" {
        stats.error_counts += 1;
    }

    for keyword in stats.keywords.iter_mut() {
        if keyword.id == id {
            let mut kwl = keyword_db.lock().await;
            let keyword_hm_key = format!("{}{}", account, id);
            let keyword_logs = kwl.entry(keyword_hm_key).or_insert(Vec::new());

            keyword_logs.push(ss.clone());

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
    db: StatsDb,
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
