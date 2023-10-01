use rocket::serde::{Serialize, Deserialize};
use rocket_db_pools::Database;
use rocket_db_pools::sqlx::{self, Row, sqlite::SqliteRow};
use chrono::NaiveDateTime;
use uuid::Uuid;

pub const SQLITE_DATE_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

#[derive(Database)]
#[database("twitter")]
pub struct Pool(pub sqlx::SqlitePool);

pub mod entities {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(crate = "rocket::serde")]
    pub struct User {
        pub id: String,
        pub username: String,
        pub created_at: NaiveDateTime
    }

    impl From<SqliteRow> for User {
        fn from(row: SqliteRow) -> Self {
            let id = row.get("id");
            let username = row.get("username");
            let created_at = NaiveDateTime::parse_from_str(row.get("created_at"), SQLITE_DATE_FORMAT).unwrap();

            Self { id, username, created_at }
        }
    }

    impl User {
        pub fn new<Str: Into<String>>(username: Str) -> Self {
            let id = Uuid::new_v4().to_string();
            let username = username.into();
            let created_at = chrono::offset::Local::now().naive_utc();

            Self { id, username, created_at }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(crate = "rocket::serde")]
    pub struct Post {
        pub id: String,
        pub user_id: String,
        pub message: String,
        pub created_at: NaiveDateTime,
    }

    impl From<SqliteRow> for Post {
        fn from(row: SqliteRow) -> Self {
            let id = row.get("id");
            let user_id = row.get("user_id");
            let message = row.get("message");
            let created_at = NaiveDateTime::parse_from_str(row.get("created_at"), SQLITE_DATE_FORMAT).unwrap();

            Self { id, user_id, message, created_at }
        }
    }

    impl Post {
        pub fn new<Str: Into<String>>(user_id: Str, message: Str) -> Self {
            let id = Uuid::new_v4().to_string();
            let user_id = user_id.into();
            let message = message.into();
            let created_at = chrono::offset::Local::now().naive_utc();

            Self { id, user_id, message, created_at }
        }
    }
}