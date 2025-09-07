#![cfg(feature = "ssr")]

use sqlx::{query, SqlitePool};
use std::sync::LazyLock;

pub static POOL: LazyLock<SqlitePool> =
    LazyLock::new(|| SqlitePool::connect_lazy("sqlite://db.sqlite").unwrap());

pub async fn prep_db() {
    query(r#"CREATE TABLE IF NOT EXISTS "users" (
    	"username"	TEXT NOT NULL UNIQUE,
	    "project"	TEXT,
    	"monday"	TEXT,
    	"tuesday"	TEXT,
    	"wednesday"	TEXT,
    	"thursday"	TEXT,
    	"friday"	TEXT,
    	"saturday"	TEXT,
    	"sunday"	TEXT,
        PRIMARY KEY("username")
    );"#).execute(&*POOL).await.unwrap();
}