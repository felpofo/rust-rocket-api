#[macro_use] extern crate rocket;
use rocket::http::Status;

#[get("/")]
fn index() -> (Status, ()) {
    (Status::Ok, ())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

#[cfg(test)]
mod tests {
    use super::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    #[test]
    fn route_index() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let response = client.get(uri!("/")).dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body().is_none(), true);
    }
}