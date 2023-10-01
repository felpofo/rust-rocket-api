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
}

#[cfg(test)]
mod tests {
    use super::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn index() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let response = client.get(uri!("/")).dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body().is_none(), true);
    }
}