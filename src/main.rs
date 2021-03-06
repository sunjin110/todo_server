extern crate todo_server;

use todo_server::controller::router;

use actix_web::{App, HttpServer};
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
    )
    .expect("Failed to create a table `todo`");

    println!("todo server start -> localhost:7777");

    // server
    HttpServer::new(move || {
        App::new()
            .service(router::index)
            .service(router::add_todo)
            .service(router::delete_todo)
            .data(pool.clone())
    })
    .bind("127.0.0.1:7777")?
    .run()
    .await?;
    Ok(())
}
