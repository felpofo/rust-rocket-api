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
}