extern crate todo_server;


use todo_server::common::error::AlmaError;
use todo_server::repository::template::{TodoEntry, IndexTemplate, render};

use actix_web::{get, App, HttpResponse, HttpServer, web};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {

    // db
    let manager = SqliteConnectionManager::file("todo.db");
    let pool = Pool::new(manager).expect("Failed to initialize the connection pool.");
    let conn = pool
        .get()
        .expect("Failed to get the connection from the pool.");
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            text TEXT NOT NULL
        )",
        params![],
    ).expect("Failed to create a table `todo`");

    // server
    HttpServer::new(move || App::new().service(index).data(pool.clone()))
        .bind("127.0.0.1:7777")?
        .run()
        .await?;
    Ok(())
}

#[get("/")]
async fn index(db: web::Data<Pool<SqliteConnectionManager>>) -> Result<HttpResponse, AlmaError> {

    // db connect
    let conn = db.get()?;

    // SQL文をPrepared Statementに変換
    let mut statement = conn.prepare("SELECT id, text FROM todo")?;

    // Prepared StatementとなっているSQL文を実行し、結果をTodoEntryに変換する
    let rows = statement.query_map(params![], |row| {
        let id = row.get(0)?;
        let text = row.get(1)?;
        Ok(TodoEntry{id, text})
    })?;


    let mut entries = Vec::new();
    for row in rows {
        entries.push(row?);
    }


    let html = IndexTemplate{entries};
    let response_body = render(html)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(response_body))
}