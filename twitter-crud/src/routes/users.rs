use rocket::Route;
use rocket::http::{Status, ContentType};
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;
use rocket_db_pools::{sqlx, Connection};
use crate::database::Pool;
use crate::database::entities::User;

pub struct Users;

impl Into<Vec<Route>> for Users {
    fn into(self) -> Vec<Route> {
        routes![users, user, create_user, delete_user, update_user]
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct RequestUserJson {
    username: String
}

#[get("/")]
async fn users(mut db: Connection<Pool>) -> Json<Vec<User>> {
    let rows = sqlx::query("SELECT * FROM users")
        .fetch_all(&mut *db)
        .await
        .unwrap();

    let users: Vec<User> = rows
        .into_iter()
        .map(User::from)
        .collect();

    Json(users)
}

#[get("/<username>")]
async fn user(mut db: Connection<Pool>, username: &str) -> (Status, Option<Json<User>>) {
    let result = sqlx::query("SELECT * FROM users WHERE username = $1")
        .bind(username)
        .fetch_one(&mut *db)
        .await;

    if let Ok(row) = result {
        let user = User::from(row);

        return (Status::Found, Some(Json(user)));
    }

    (Status::NotFound, None)
}

#[post("/", format = "json", data = "<user>")]
async fn create_user(mut db: Connection<Pool>, user: Json<RequestUserJson>) -> Result<(Status, (ContentType, Json<User>)), Status> {
    if user.username.len() > 32 || user.username.contains(' ') {
        return Err(Status::BadRequest);
    }

    let user = User::new(&user.username);

    let result = sqlx::query("INSERT INTO users (id, username) VALUES ($1, $2)")
        .bind(&user.id)
        .bind(&user.username)
        .execute(&mut *db)
        .await;

    match result {
        Ok(_) => return Ok((Status::Created, (ContentType::JSON, Json(user)))),
        Err(err) => {
            match err {
                sqlx::Error::Database(database) => {
                    if database.message().starts_with("UNIQUE") {
                        return Err(Status::BadRequest);
                    }
                }
                _ => {}
            }
        }
    };

    Err(Status::NotImplemented)
}

#[patch("/<username>", format = "json", data = "<new>")]
async fn update_user(mut db: Connection<Pool>, username: String, new: Json<RequestUserJson>) -> Result<(Status, (ContentType, Json<User>)), Status> {
    if new.username.len() > 32 || new.username.contains(' ') {
        return Err(Status::BadRequest)
    }

    let result = sqlx::query("SELECT * FROM users WHERE username = $1")
        .bind(&username)
        .fetch_one(&mut *db)
        .await;

    match result {
        Ok(row) => {
            let mut user = User::from(row);

            let result = sqlx::query("UPDATE users SET username = $1 WHERE id = $2")
                .bind(&new.username)
                .bind(&user.id)
                .execute(&mut *db)
                .await;

            if let Ok(result) = result {
                if result.rows_affected() > 0 {
                    user.username = new.username.clone();
                    return Ok((Status::Accepted, (ContentType::JSON, Json(user))));
                }
            }
        }
        Err(_) => return Err(Status::NotFound),
    };

    Err(Status::NotImplemented)
}

#[delete("/", format = "json", data = "<user>")]
async fn delete_user(mut db: Connection<Pool>, user: Json<RequestUserJson>) -> Result<Status, Status> {
    let result = sqlx::query("DELETE FROM users WHERE username = $1")
        .bind(&user.username)
        .execute(&mut *db)
        .await;

    match result {
        Ok(result) => {
            return if result.rows_affected() > 0 {
                Ok(Status::Ok)
            } else {
                Err(Status::NotFound)
            }
        }
        Err(_) => {}
    };

    Err(Status::NotImplemented)
}
