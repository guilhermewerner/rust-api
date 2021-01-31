#![feature(proc_macro_hygiene, decl_macro)]

use std::env;
use dotenv::dotenv;

use rocket::{get, routes, catch, catchers};
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};

use mysql::*;
use mysql::prelude::*;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Email {
    address: String,
    user_id: String,
    is_confirmed: bool,
    is_primary: bool,
}

#[get("/")]
fn hello() -> JsonValue {
    return json!({ "TribuFu": "Hello World" });
}

#[get("/emails")]
fn emails() -> JsonValue {
    dotenv().ok();

    let MYSQL_HOST: String = env::var("MYSQL_HOST").expect("Missing MYSQL_HOST in Environment");
    let MYSQL_PORT: String = env::var("MYSQL_PORT").expect("Missing MYSQL_PORT in Environmment");
    let MYSQL_USERNAME: String = env::var("MYSQL_USERNAME").expect("Missing MYSQL_USERNAME in Environmment");
    let MYSQL_PASSWORD: String = env::var("MYSQL_PASSWORD").expect("Missing MYSQL_PASSWORD in Environmment");
    let MYSQL_DATABASE: String = env::var("MYSQL_DATABASE").expect("Missing MYSQL_DATABASE in Environmment");

    let url = format!("mysql://{}:{}@{}:{}/{}", MYSQL_USERNAME, MYSQL_PASSWORD, MYSQL_HOST, MYSQL_PORT, MYSQL_DATABASE);

    let pool = Pool::new(url).unwrap();

    let mut connection = pool.get_conn().unwrap();

    // Let's select emails from database. Type inference should do the trick here.
    let emails = connection.query_map(
            "SELECT Address, UserId, IsConfirmed, IsPrimary from Email",
            |(address, user_id, is_confirmed, is_primary)| {
                Email { address, user_id, is_confirmed, is_primary }
            },
        ).unwrap();

    return json!(emails);
}

#[catch(404)]
fn not_found() -> JsonValue {
    return json!({
        "Error": "RESOURCE_NOT_FOUND"
    });
}

fn main() {
    rocket::ignite()
        .mount("/", routes![hello, emails])
        .register(catchers![not_found])
        .launch();
}
