#[macro_use] extern crate rocket;

use rocket::serde::{Serialize, Deserialize, json::Json};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Serialize)]
struct Resp {
    msg: String,
    challenge: String,
}

#[post("/interactivity", data = "<data>")]
fn interactivity(data: String) -> Json<Resp> {
    println!("rcv: {}", data);
    Json(Resp {
        msg: "ok".into(),
        challenge: "".into(),
    })
}

#[derive(Deserialize, Debug)]
struct ChallengeReq {
    token: String,

    challenge: String,

    #[serde(rename(serialize = "type", deserialize = "type"))]
    tp: String,
}

#[post("/event", data = "<req>")]
fn event(req: Json<ChallengeReq>) -> Json<Resp> {
    println!("evt: {:?}", req);

    let data = req.challenge.clone();

    Json(Resp {
        msg: "ok".into(),
        challenge: data,
    })
}

#[post("/slash/vul", data = "<data>")]
fn vul(data: String) -> Json<Resp> {
    println!("vul: {}", data);
    Json(Resp {
        msg: "ok".into(),
        challenge: "".into(),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/api/v1", routes![interactivity, vul, event])
}
