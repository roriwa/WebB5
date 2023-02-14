use rocket::fs::{FileServer, relative};
use rocket::launch;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("public")))
}
