#![allow(non_snake_case)]

mod Controllers;

use actix_web::{middleware, App, HttpServer};
use std::env;
use std::io::Result;

use Controllers::Hello;

pub async fn Main() -> Result<()> {
    env::set_var("FRAMEWORK_LOG_LEVEL", "debug");

    Framework_Log::Init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .service(Hello::Index)
    })
    .bind("localhost:5000")?
    .run()
    .await
}
