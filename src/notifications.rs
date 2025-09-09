#![cfg(feature = "ssr")]

use axum::http::HeaderMap;
use chrono::{Datelike, Utc};
use reqwest::Client;
use serde_json::Value;
use sqlx::{query, Row};
use std::time::Duration;
use tokio::time::interval;

use crate::db::POOL;

pub async fn notifications() {
    let mut interval = interval(Duration::from_secs(1));
    let mut last = 0;
    loop {
        let monday = Utc::now()
            .date_naive()
            .checked_sub_signed(chrono::Duration::days(
                Utc::now().weekday().num_days_from_monday() as i64,
            ))
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let time_passed = Utc::now()
            .naive_utc()
            .signed_duration_since(monday)
            .num_minutes();
        if time_passed != last {
            dbg!(time_passed);
            last = time_passed;
            let rows = match query("SELECT username, project, 
                CASE
                    WHEN monday = ?1 THEN monday_goal
                    WHEN tuesday = ?1 THEN tuesday_goal
                    WHEN wednesday = ?1 THEN wednesday_goal
                    WHEN thursday = ?1 THEN thursday_goal
                    WHEN friday = ?1 THEN friday_goal
                    WHEN saturday = ?1 THEN saturday_goal
                    WHEN sunday = ?1 THEN (monday_goal + tuesday_goal + wednesday_goal + thursday_goal + friday_goal + saturday_goal + sunday_goal)
                END AS goal,
                CASE
                    WHEN monday = ?1 THEN 0
                    WHEN tuesday = ?1 THEN monday
                    WHEN wednesday = ?1 THEN thursday
                    WHEN thursday = ?1 THEN wednesday
                    WHEN friday = ?1 THEN thursday
                    WHEN saturday = ?1 THEN friday
                    WHEN sunday = ?1 THEN saturday
                END AS last_time,
                CASE
                    WHEN sunday = ?1 THEN 1
                    ELSE 0
                END AS sunday
                FROM users WHERE ?1 IN (monday, tuesday, wednesday, thursday, friday, saturday, sunday)"
            ).bind(time_passed).fetch_all(&*POOL).await {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("{}", e);
                    continue;
                },
            };
            for i in rows {
                let username: String = i.get("username");
                let project: String = i.get("project");
                if project == "".to_string() {
                    let mut headers = HeaderMap::new();
                    headers.append("Title", "Set your project.".parse().unwrap());
                    let client = Client::new();
                    client
                        .post(format!("https://ntfy.tim.hackclub.app/{}", username))
                        .headers(headers)
                        .body("You haven't set a project for this week.")
                        .send()
                        .await
                        .unwrap();
                    return;
                }
                let goal: i64 = i.get("goal");

                let client = Client::new();
                let sunday: u8 = i.get("sunday");
                let res = match if sunday == 0 {
                    client
                        .get(format!(
                            "https://hackatime.hackclub.com/api/v1/users/{}/stats?features=projects&start_date={}",
                            username,
                            monday.checked_add_signed(chrono::Duration::minutes(i.get("last_time"))).unwrap().and_utc().to_rfc3339()
                        ))
                        .send()
                        .await
                } else {
                    let today = Utc::now().date_naive();
                    let week_start = today
                        - chrono::Duration::days(today.weekday().num_days_from_monday() as i64);
                    client
                        .get(format!(
                            "https://hackatime.hackclub.com/api/v1/users/{}/stats?features=projects&start_date={}",
                            username,
                            week_start.and_hms_opt(4, 0, 0).unwrap().and_utc().to_rfc3339()
                        ))
                        .send()
                        .await
                } {
                    Ok(res) => res,
                    Err(_) => continue,
                };

                if !res.status().is_success() {
                    continue;
                }
                let data: Value = match res.json().await {
                    Ok(d) => d,
                    Err(_) => continue,
                };

                let seconds_spend = match get_project(data, project.clone()) {
                    Some(s) => s,
                    _ => 0,
                };

                let sec_over_goal = seconds_spend - goal * 60 * 60;

                let mut headers = HeaderMap::new();

                let msg = if sunday == 0 {
                    if sec_over_goal >= 0 {
                        headers.append("Title", "Good Job!".parse().unwrap());
                        headers.append("Tags", "tada".parse().unwrap());
                        format!(
                            "You worked for {} more than your goal was.",
                            sec_to_hms(sec_over_goal)
                        )
                    } else {
                        headers.append("Title", "You need to lock in!".parse().unwrap());
                        headers.append("Tags", "warning".parse().unwrap());
                        format!(
                            "You worked for {} less than your goal was.",
                            sec_to_hms(sec_over_goal)
                        )
                    }
                } else {
                    if sec_over_goal >= 0 {
                        headers.append(
                            "Title",
                            "Good Job, don't forget to submit!".parse().unwrap(),
                        );
                        headers.append("Tags", "tada".parse().unwrap());
                        format!(
                            "You worked for {} more than your weekly goal was.",
                            sec_to_hms(sec_over_goal)
                        )
                    } else {
                        headers.append("Title", "You need to lock in!".parse().unwrap());
                        headers.append("Tags", "warning".parse().unwrap());
                        format!(
                            "You're {} short of your weekly goal and you need to submit soon.",
                            sec_to_hms(sec_over_goal)
                        )
                    }
                };

                let client = Client::new();
                client
                    .post(format!("https://ntfy.tim.hackclub.app/{}", username))
                    .headers(headers)
                    .body(msg)
                    .send()
                    .await
                    .unwrap();
            }
            if time_passed == 0 {
                match query("UPDATE users SET project = ''").execute(&*POOL).await {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("{}", e);
                        continue;
                    }
                }
            }
        }
        interval.tick().await;
    }
}

fn get_project(data: Value, project: String) -> Option<i64> {
    for i in data.get("data")?.get("projects")?.as_array()? {
        if i.get("name")?.as_str()?.to_string() == project {
            return i.get("total_seconds")?.as_i64();
        }
    }
    None
}

fn sec_to_hms(sec: i64) -> String {
    let mut out = String::new();
    let mut sec = sec.unsigned_abs();
    if sec > 60 * 60 {
        out.push_str(&format!("{}h ", sec / (60 * 60)));
        sec = sec - (sec / (60 * 60)) * 60 * 60;
    }
    if sec > 60 {
        out.push_str(&format!("{}m ", sec / 60));
        sec = sec - (sec / (60)) * 60;
    }
    if sec > 0 {
        out.push_str(&format!("{}s ", sec));
    }
    out
}
