#[macro_use] extern crate rocket;
use rand::prelude::*;
use rocket::fs::FileServer;

#[get("/random/<min>/<max>")]
fn random(min: u64, max: u64) -> String {
    let mut rng = rand::thread_rng();
    let num: u64 = rng.gen_range(min..max);
    format!("{}", num)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("www/public"))
        .mount("/api/", routes![random])
}