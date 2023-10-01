use rocket::Route;
use rocket::http::{Status, ContentType};
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;
use rocket_db_pools::{sqlx, Connection};
use crate::database::Pool;
use crate::database::entities::Post;

pub struct Posts;

impl Into<Vec<Route>> for Posts {
    fn into(self) -> Vec<Route> {
        routes![posts, post, create_post, delete_post]
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct RequestPostJson {
    user_id: String,
    message: String
}

#[get("/")]
async fn posts(mut db: Connection<Pool>) -> Json<Vec<Post>> {
    let rows = sqlx::query("SELECT * FROM posts")
        .fetch_all(&mut *db)
        .await
        .unwrap();

    let posts: Vec<Post> = rows
        .into_iter()
        .map(Post::from)
        .collect();

    Json(posts)
}

#[get("/<id>")]
async fn post(mut db: Connection<Pool>, id: &str) -> (Status, Option<Json<Post>>) {
    let result = sqlx::query("SELECT * FROM posts WHERE id = $1")
        .bind(id)
        .fetch_one(&mut *db)
        .await;

    if let Ok(row) = result {
        let post = Post::from(row);

        return (Status::Found, Some(Json(post)))
    }

    (Status::NotFound, None)
}

#[post("/", format = "json", data = "<post>")]
async fn create_post(mut db: Connection<Pool>, post: Json<RequestPostJson>) -> Result<(Status, (ContentType, Json<Post>)), Status> {
    if post.message.len() > 256 {
        return Err(Status::BadRequest);
    }

    let post = Post::new(&post.user_id, &post.message);

    let result = sqlx::query("INSERT INTO posts (id, user_id, message) VALUES ($1, $2, $3)")
        .bind(&post.id)
        .bind(&post.user_id)
        .bind(&post.message)
        .execute(&mut *db)
        .await;

    match result {
        Ok(_) => return Ok((Status::Created, (ContentType::JSON, Json(post)))),
        Err(_) => {}
    };

    Err(Status::NotImplemented)
}

#[delete("/<id>")]
async fn delete_post(mut db: Connection<Pool>, id: &str) -> Result<Status, Status> {
    let result = sqlx::query("DELETE FROM posts WHERE id = $1")
        .bind(id)
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