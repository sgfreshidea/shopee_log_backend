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

    let path = crate::helpers::sanitize(&crate::helpers::current_time_string()) + ".json";
    let mut new_file = File::create(path).unwrap();

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
                let keyword_logs = kwl.entry(keyword_hm_key).or_insert(Vec::with_capacity(100));

                if ii.logs.is_some() {
                    let slice = ii.logs.as_ref().unwrap();
                    keyword_logs.extend_from_slice(&slice);
                    jj.log_counts += slice.len() as u64;
                }
            }
        }

        if pushed == false {
            stats.keywords.push(KeywordStat {
                id: ii.id,
                last_updated_at: crate::helpers::current_time_string(),
                name: ii.name.clone(),
                keyword: ii.keyword.clone(),
                placement: ii.placement,
                running: ii.running,
                error_counts: 0,
                log_counts: 0,
                ads_running: ii.ads_running,
                ads_position: ii.ads_position,
                current_price: ii.current_price,
            });
        }
    }

    Ok(warp::reply::json(&json!({
        "type": "success",
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
            keyword.last_updated_at = crate::helpers::current_time_string();

            let mut kwl = keyword_db.lock().await;
            let keyword_hm_key = format!("{}{}", account, id);
            let keyword_logs = kwl.entry(keyword_hm_key).or_insert(Vec::with_capacity(100));

            keyword.log_counts += 1;
            keyword_logs.push(ss.clone());

            if ss.r#type == "error" {
                keyword.error_counts = keyword.error_counts + 1;
            }
        }
    }

    Ok(warp::reply::json(&json!({
        "type": "success",
    })))
}

pub async fn update_keyword_stat(
    account: String,
    ss: KeywordStatInput,
    db: StatsDb,
) -> Result<impl warp::Reply, Infallible> {
    let mut lock = db.lock().await;
    let stats = lock.entry(account).or_insert(Stat::new());
    stats.last_updated_at = crate::helpers::current_time_string();

    for keyword in stats.keywords.iter_mut() {
        if keyword.id == ss.id {
            keyword.last_updated_at = crate::helpers::current_time_string();

            if let Some(ru) = ss.running {
                keyword.running = Some(ru);
            }

            // Todo use if let some feeling lazy atm.
            if ss.ads_running.is_some() {
                keyword.ads_running = ss.ads_running
            }

            if ss.ads_position.is_some() {
                keyword.ads_position = ss.ads_position
            }

            if let Some(cp) = ss.current_price {
                keyword.current_price = Some(cp);
            }
        }
    }

    Ok(warp::reply::json(&json!({
        "type": "success",
    })))
}
