use actix_web::get;

#[get("/")]
pub async fn Index() -> &'static str {
    return "Hello World\n";
}
