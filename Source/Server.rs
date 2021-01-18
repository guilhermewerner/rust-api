#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::json::{Json, JsonValue};

#[get("/")]
fn hello() -> JsonValue {
    return json!({ "TribuFu": "Hello World" });
}

#[catch(404)]
fn not_found() -> JsonValue {
    return json!({
        "Error": "RESOURCE_NOT_FOUND"
    });
}

fn rocket() -> rocket::Rocket {
    return rocket::ignite()
        .mount("/", routes![hello])
        .register(catchers![not_found]);
}

fn main() {
    rocket().launch();
}
