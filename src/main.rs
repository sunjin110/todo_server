extern crate todo_server;


use todo_server::common::error::AlmaError;
use todo_server::repository::template::{TodoEntry, IndexTemplate, render};

use actix_web::{get, App, HttpResponse, HttpServer};

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(index))
        .bind("127.0.0.1:7777")?
        .run()
        .await?;
    Ok(())
}

#[get("/")]
async fn index() -> Result<HttpResponse, AlmaError> {
    let mut entries = Vec::new();
    entries.push(TodoEntry{
        id: 1,
        text: "First".to_string(),
    });

    entries.push(TodoEntry{
        id: 2,
        text: "Second".to_string(),
    });

    let html = IndexTemplate{entries};
    let response_body = render(html)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(response_body))
}