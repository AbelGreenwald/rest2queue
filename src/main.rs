#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate serde_json;

use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct Message {
    name: String,
    id: String,
}

#[derive(Deserialize)]
struct Request {
    version: i32,
    contents: Message,
}

#[post("/index", format = "application/json", data = "<message>")]
fn new(message: Json<Request>) {
    println!("{}", message.version);
    let instance = json!(&message.contents);
    drop(instance);
}

#[get("/health")]
fn health() {}

fn main() {
    rocket::ignite()
        .mount("/", routes![new])
        .mount("/", routes![health])
        .launch();
}
