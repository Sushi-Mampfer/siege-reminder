use chrono::{Datelike, Duration, Utc};
use leptos::{prelude::ServerFnError, server};
use reqwest::{Client, StatusCode};
use serde_json::Value;

use crate::datatypes::{Data, Project, Settings};

#[cfg(feature = "ssr")]
use crate::db::POOL;
#[cfg(feature = "ssr")]
use sqlx::{query, Row};

#[server]
pub async fn set_project(username: String, project: String) -> Result<(), ServerFnError> {
    match query("UPDATE users SET project = ? WHERE username = ?")
        .bind(project)
        .bind(username)
        .execute(&*POOL)
        .await
    {
        Ok(_) => Ok(()),
        Err(_) => return Err(ServerFnError::new("Database error")),
    }
}

#[server]
pub async fn set_times(username: String, times: Settings) -> Result<(), ServerFnError> {
    match query("UPDATE users SET monday = ?, tuesday = ?, wednesday = ?, thursday = ?, friday = ?, saturday = ?, sunday = ? WHERE username = ?").bind(times.monday).bind(times.tuesday).bind(times.wednesday).bind(times.thursday).bind(times.friday).bind(times.saturday).bind(times.sunday).bind(username).execute(&*POOL).await {
        Ok(_) => Ok(()),
        Err(_) => return Err(ServerFnError::new("Database error")),
    }
}

#[server]
pub async fn query_data(username: String) -> Result<Data, ServerFnError> {
    let projects = query_projects(username.clone()).await?;
    let row = match query("SELECT * FROM users WHERE username = ?")
        .bind(username)
        .fetch_one(&*POOL)
        .await
    {
        Ok(r) => r,
        Err(sqlx::Error::RowNotFound) => todo!(),
        Err(_) => return Err(ServerFnError::new("Database error")),
    };
    let primary: String = match row.try_get("project") {
        Ok(p) => p,
        Err(_) => "".to_string(),
    };

    Ok(Data {
        projects,
        primary,
        settings: Settings {
            monday: match row.try_get("monday") {
                Ok(p) => p,
                Err(_) => return Err(ServerFnError::new("Database error")),
            },
            tuesday: match row.try_get("tuesday") {
                Ok(p) => p,
                Err(_) => return Err(ServerFnError::new("Database error")),
            },
            wednesday: match row.try_get("wednesday") {
                Ok(p) => p,
                Err(_) => return Err(ServerFnError::new("Database error")),
            },
            thursday: match row.try_get("thursday") {
                Ok(p) => p,
                Err(_) => return Err(ServerFnError::new("Database error")),
            },
            friday: match row.try_get("friday") {
                Ok(p) => p,
                Err(_) => return Err(ServerFnError::new("Database error")),
            },
            saturday: match row.try_get("saturday") {
                Ok(p) => p,
                Err(_) => return Err(ServerFnError::new("Database error")),
            },
            sunday: match row.try_get("sunday") {
                Ok(p) => p,
                Err(_) => return Err(ServerFnError::new("Database error")),
            },
        },
    })
}

async fn query_projects(username: String) -> Result<Vec<Project>, ServerFnError> {
    let today = Utc::now().date_naive();
    let week_start = today - Duration::days(today.weekday().num_days_from_monday() as i64);
    let client = Client::new();
    let res = match client
        .get(format!(
            "https://hackatime.hackclub.com/api/v1/users/{}/stats?features=projects&start_date={}",
            username,
            week_start.format("%d-%m").to_string()
        ))
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => return Err(ServerFnError::new(e)),
    };
    if res.status() == StatusCode::NOT_FOUND {
        return Err(ServerFnError::new("Username not found"));
    } else if !res.status().is_success() {
        return Err(ServerFnError::new(
            res.status()
                .canonical_reason()
                .unwrap_or_else(|| "Unknown error"),
        ));
    }

    let data: Value = match res.json().await {
        Ok(d) => d,
        Err(_) => return Err(ServerFnError::new("Failed to deserialize response data")),
    };

    match parse_projects(data) {
        Some(d) => Ok(d),
        None => Err(ServerFnError::new("Failed to parse response data")),
    }
}

fn parse_projects(data: Value) -> Option<Vec<Project>> {
    let mut out = Vec::new();

    let projects = data.get("data")?.get("projects")?;
    for i in projects.as_array()? {
        out.push((
            i.get("name")?.as_str()?.to_string(),
            i.get("text")?.as_str()?.to_string(),
            i.get("total_seconds")?.as_i64()?,
        ));
    }
    out.sort_by(|a, b| a.2.cmp(&b.2));
    Some(out.iter().map(|p| Project { name: p.0.clone(), time: p.1.clone() }).collect())
}
