use actix_web::{get, App, HttpResponse, HttpServer};
extern crate todo;
use todo::my_error;
use todo::entry;
use todo::render_temp;
use askama::Template;

#[get("/")]
async fn index() -> Result<HttpResponse, my_error::MyError> {
    let mut entries = Vec::new();
    entries.push(entry::TodoEntry {
        id: 1,
        text: "First Entry".to_string(),
    });
    entries.push(entry::TodoEntry {
        id: 2,
        text: "Second Entry".to_string(),
    });

    let html = render_temp::IndexTemplate {entries};
    let response_body = html.render()?;

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}
