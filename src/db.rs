#![cfg(feature = "ssr")]

use sqlx::{query, SqlitePool};
use std::sync::LazyLock;

pub static POOL: LazyLock<SqlitePool> =
    LazyLock::new(|| SqlitePool::connect_lazy("sqlite://db.sqlite").unwrap());

pub async fn prep_db() {
    query(r#"CREATE TABLE IF NOT EXISTS "users" (
    	"username"	TEXT NOT NULL UNIQUE,
	    "project"	TEXT,
    	"monday"	INTEGER,
		"monday_goal"	INTEGER,
    	"tuesday"	INTEGER,
    	"tuesday_goal"	INTEGER,
    	"wednesday"	INTEGER,
    	"wednesday_goal"	INTEGER,
    	"thursday"	INTEGER,
    	"thursday_goal"	INTEGER,
    	"friday"	INTEGER,
    	"friday_goal"	INTEGER,
    	"saturday"	INTEGER,
    	"saturday_goal"	INTEGER,
    	"sunday"	INTEGER,
    	"sunday_goal"	INTEGER,
        PRIMARY KEY("username")
    );"#).execute(&*POOL).await.unwrap();
}