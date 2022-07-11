use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listeners: Vec<TcpListener>) -> Result<Server, std::io::Error> {
    let server = {
        let mut server =
            HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)));
        for listener in listeners {
            server = server.listen(listener)?;
        }
        server.run()
    };
    Ok(server)
}
