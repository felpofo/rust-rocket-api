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
            .get(uri!("/"))
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert!(response.body().is_none());
    }

    #[test]
    fn b_get_all_users() {
        let client = Client::tracked(super::rocket()).expect("valid rocket instance");

        let response = client
            .get(uri!("/users"))
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
            .post(uri!("/users"))
            .header(ContentType::JSON)
            .body(r#"{"username":"test"}"#)
            .dispatch();

        assert_eq!(response.status(), Status::Created);
        assert!(response.body().is_some());
        
        let user = response.into_json::<User>().unwrap();

        assert!(user.id.len() == 36);
        assert_eq!(user.username, "test");
    }

    #[test]
    fn d_get_user() {
        use super::database::entities::User;

        let client = Client::tracked(super::rocket()).expect("valid rocket instance");

        let response = client
            .get(uri!("/users/test"))
            .dispatch();

        assert_eq!(response.status(), Status::Found);
        assert!(response.body().is_some());
        
        let user = response.into_json::<User>().unwrap();

        assert!(user.id.len() == 36);
        assert_eq!(user.username, "test");
    }

    #[test]
    fn e_update_user() {
        use super::database::entities::User;

        let client = Client::tracked(super::rocket()).expect("valid rocket instance");

        let response = client
            .patch(uri!("/users/test"))
            .header(ContentType::JSON)
            .body(r#"{"username":"after"}"#)
            .dispatch();

        assert_eq!(response.status(), Status::Accepted);
        assert!(response.body().is_some());
        
        let user = response.into_json::<User>().unwrap();

        assert!(user.id.len() == 36);
        assert_eq!(user.username, "after");
    }

    #[test]
    fn f_delete_user() {
        let client = Client::tracked(super::rocket()).expect("valid rocket instance");

        let response = client
            .delete(uri!("/users/after"))
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert!(response.body().is_none());
    }
}