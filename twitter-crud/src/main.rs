#[macro_use] extern crate rocket;

mod routes;
pub mod database;

#[get("/")]
fn index() {}

#[launch]
fn rocket() -> _ {
    use rocket_db_pools::Database;

    rocket::build()
        .attach(database::Pool::init())
        .mount("/", routes![index])
        .mount("/users", routes::Users)
        .mount("/posts", routes::Posts)
}

#[cfg(test)]
mod tests {
    use rocket::http::{Status, ContentType};
    use rocket::local::blocking::Client;

    #[test]
    fn a_homepage() {
        let client = Client::tracked(super::rocket()).expect("valid rocket instance");

        let response = client
            .get("/")
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert!(response.body().is_none());
    }

    #[test]
    fn b_get_all_users() {
        let client = Client::tracked(super::rocket()).expect("valid rocket instance");

        let response = client
            .get("/users")
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert!(response.body().is_some());
        assert_eq!(response.into_string().unwrap(), "[]");
    }

    #[test]
    fn c_create_user() {
        use super::database::entities::User;

        let client = Client::tracked(super::rocket())
            .expect("valid rocket instance");

        let response = client
            .post("/users")
            .header(ContentType::JSON)
            .body(r#"{"username":"before"}"#)
            .dispatch();

        assert_eq!(response.status(), Status::Created);
        assert!(response.body().is_some());

        let user: User = response.into_json().unwrap();

        assert!(user.id.len() == 36);
        assert_eq!(user.username, "before");
    }

    #[test]
    fn d_get_user() {
        use super::database::entities::User;

        let client = Client::tracked(super::rocket()).expect("valid rocket instance");

        let response = client
            .get("/users/before")
            .dispatch();

        assert_eq!(response.status(), Status::Found);
        assert!(response.body().is_some());

        let user: User = response.into_json().unwrap();

        assert!(user.id.len() == 36);
        assert_eq!(user.username, "before");
    }

    #[test]
    fn e_update_user() {
        use super::database::entities::User;

        let client = Client::tracked(super::rocket()).expect("valid rocket instance");

        let response = client
            .patch("/users/before")
            .header(ContentType::JSON)
            .body(r#"{"username":"after"}"#)
            .dispatch();

        assert_eq!(response.status(), Status::Accepted);
        assert!(response.body().is_some());

        let user: User = response.into_json().unwrap();

        assert!(user.id.len() == 36);
        assert_eq!(user.username, "after");
    }

    #[test]
    fn f_get_all_posts() {
        let client = Client::tracked(super::rocket()).expect("valid rocket instance");

        let response = client
            .get("/posts")
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert!(response.body().is_some());
        assert_eq!(response.into_string().unwrap(), "[]");
    }

    #[test]
    fn g_create_and_delete_post() {
        use super::database::entities::{User, Post};

        let client = Client::tracked(super::rocket())
            .expect("valid rocket instance");

        let response = client
            .get("/users/after")
            .dispatch();

        let user: User = response.into_json().unwrap();

        let response = client
            .post("/posts")
            .header(ContentType::JSON)
            .body(format!(r#"{{"user_id":"{}","message":"test message"}}"#, user.id))
            .dispatch();

        assert_eq!(response.status(), Status::Created);
        assert!(response.body().is_some());

        let post: Post = response.into_json().unwrap();

        assert!(post.id.len() == 36);
        assert!(post.user_id == user.id);
        assert_eq!(post.message, "test message");

        let response = client
            .delete(format!("/posts/{}", post.id))
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert!(response.body().is_none());
    }

    #[test]
    fn h_get_post() {
        use super::database::entities::{User, Post};

        let client = Client::tracked(super::rocket())
            .expect("valid rocket instance");

        let response = client
            .get("/users/after")
            .dispatch();

        let user: User = response.into_json().unwrap();

        let response = client
            .post("/posts")
            .header(ContentType::JSON)
            .body(format!(r#"{{"user_id":"{}","message":"test message"}}"#, user.id))
            .dispatch();

        assert_eq!(response.status(), Status::Created);
        assert!(response.body().is_some());

        let post: Post = response.into_json().unwrap();

        assert!(post.id.len() == 36);
        assert!(post.user_id == user.id);
        assert_eq!(post.message, "test message");

        let response = client
            .get(format!("/posts/{}", post.id))
            .dispatch();

        assert_eq!(response.status(), Status::Found);
        assert!(response.body().is_some());

        let post_found: Post = response.into_json().unwrap();

        assert_eq!(post, post_found);

        let response = client
            .delete(format!("/posts/{}", post.id))
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert!(response.body().is_none());
    }

    #[test]
    fn i_delete_user() {
        let client = Client::tracked(super::rocket()).expect("valid rocket instance");

        let response = client
            .delete("/users/after")
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert!(response.body().is_none());
    }
}