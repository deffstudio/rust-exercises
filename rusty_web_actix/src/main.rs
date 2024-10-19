use actix_web::{App, HttpServer};
mod processes;
mod state;
mod to_do;
mod views;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new().configure(views::views_factory);
        return app;
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// 133
