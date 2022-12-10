#[macro_use] extern crate rocket;

use rocket::serde::{Serialize, json::Json};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Serialize)]
struct Resp<'r> {
    msg: &'r str,
}

#[post("/interactivity", data = "<data>")]
fn interactivity(data: String) -> Json<Resp<'static>> {
    println!("rcv: {}", data);
    Json(Resp { msg: "ok" })
}

#[post("/slash/vul", data = "<data>")]
fn vul(data: String) -> Json<Resp<'static>> {
    println!("vul: {}", data);
    Json(Resp { msg: "ok" })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/api/v1", routes![interactivity, vul])
}
