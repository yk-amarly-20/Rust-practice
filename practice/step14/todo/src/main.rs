use actix_web::{web, get, App, HttpResponse, HttpServer};
extern crate todo;
use todo::my_error;
use todo::entry;
use todo::render_temp;
use askama::Template;
use r2d2::Pool;
use rusqlite::params;
use r2d2_sqlite::SqliteConnectionManager;

#[get("/")]
async fn index(db: web::Data<Pool<SqliteConnectionManager>>) -> Result<HttpResponse, my_error::MyError> {
    let conn = db.get()?;
    let mut statement = conn.prepare("SELECT id, text FROM todo")?;
    let rows = statement.query_map(params![], |row| {
        let id = row.get(0)?;
        let text = row.get(1)?;
        Ok(entry::TodoEntry {id, text})
    })?;

    let mut entries = Vec::new();
    for row in rows {
        entries.push(row?);
    }
    let html = render_temp::IndexTemplate {entries};
    let response_body = html.render()?;

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
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

    HttpServer::new(move || App::new().service(index).data(pool.clone()))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}
