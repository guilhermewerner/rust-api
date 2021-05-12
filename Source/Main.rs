#![allow(non_snake_case)]

use std::io::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    return Backend::Main().await;
}
