pub mod router {

    use crate::common::error::AlmaError;
    use crate::repository::template::{render, AddParams, DeleteParams, IndexTemplate, TodoEntry};

    use actix_web::{get, http::header, post, web, HttpResponse};
    use r2d2::Pool;
    use r2d2_sqlite::SqliteConnectionManager;
    use rusqlite::params;

    #[get("/")]
    pub async fn index(
        db: web::Data<Pool<SqliteConnectionManager>>,
    ) -> Result<HttpResponse, AlmaError> {
        // db connect
        let conn = db.get()?;

        // SQL文をPrepared Statementに変換
        let mut statement = conn.prepare("SELECT id, text FROM todo")?;

        // Prepared StatementとなっているSQL文を実行し、結果をTodoEntryに変換する
        let rows = statement.query_map(params![], |row| {
            let id = row.get(0)?;
            let text = row.get(1)?;
            Ok(TodoEntry { id, text })
        })?;

        let mut entries = Vec::new();
        for row in rows {
            entries.push(row?);
        }

        let html = IndexTemplate { entries };
        let response_body = render(html)?;
        Ok(HttpResponse::Ok()
            .content_type("text/html")
            .body(response_body))
    }

    #[post("/add")]
    pub async fn add_todo(
        params: web::Form<AddParams>,
        db: web::Data<r2d2::Pool<SqliteConnectionManager>>,
    ) -> Result<HttpResponse, AlmaError> {
        let conn = db.get()?;
        conn.execute("INSERT INTO todo (text) VALUES (?)", &[&params.text])?;
        Ok(HttpResponse::SeeOther()
            .header(header::LOCATION, "/")
            .finish())
    }

    #[post("/delete")]
    pub async fn delete_todo(params: web::Form<DeleteParams>, db: web::Data<r2d2::Pool<SqliteConnectionManager>>) -> Result<HttpResponse, AlmaError> {
        let conn = db.get()?;
        conn.execute("DELETE FROM todo WHERE id=?", &[params.id])?;
        Ok(HttpResponse::SeeOther().header(header::LOCATION, "/").finish())
    }
}
