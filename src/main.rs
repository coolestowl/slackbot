#[macro_use] extern crate rocket;

use rocket::serde::{Serialize, Deserialize, json::Json};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Serialize)]
struct Resp {
    msg: String,
    chanllenge: String,
}

#[post("/interactivity", data = "<data>")]
fn interactivity(data: String) -> Json<Resp> {
    println!("rcv: {}", data);
    Json(Resp {
        msg: "ok".into(),
        chanllenge: "".into(),
    })
}

#[derive(Deserialize, Debug)]
struct ChanllengeReq {
    token: String,
    chanllenge: String,
    tp: String,
}

#[post("/event", data = "<req>")]
fn event(req: Json<ChanllengeReq>) -> Json<Resp> {
    println!("evt: {:?}", req);

    let data = req.chanllenge.clone();

    Json(Resp {
        msg: "ok".into(),
        chanllenge: data,
    })
}

#[post("/slash/vul", data = "<data>")]
fn vul(data: String) -> Json<Resp> {
    println!("vul: {}", data);
    Json(Resp {
        msg: "ok".into(),
        chanllenge: "".into(),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/api/v1", routes![interactivity, vul, event])
}
