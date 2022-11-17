use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use serde;
use std::net::TcpListener;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listeners: Vec<TcpListener>) -> Result<Server, std::io::Error> {
    let server = {
        let mut server = HttpServer::new(|| {
            App::new()
                .route("/health_check", web::get().to(health_check))
                .route("/subscriptions", web::post().to(subscribe))
        });
        for listener in listeners {
            server = server.listen(listener)?;
        }
        server.run()
    };
    Ok(server)
}

pub fn percent_encode_kvps<'a>(
    key_value_pairs: impl IntoIterator<Item = (&'a str, &'a str)>,
) -> String {
    let mut s: String = String::new();
    for (key, value) in key_value_pairs.into_iter() {
        let key: String =
            percent_encoding::utf8_percent_encode(&key, percent_encoding::NON_ALPHANUMERIC)
                .collect();
        let value: String =
            percent_encoding::utf8_percent_encode(&value, percent_encoding::NON_ALPHANUMERIC)
                .collect();
        s.push_str(&key);
        s.push('=');
        s.push_str(&value);
        s.push('&')
    }
    s.pop();
    s
}
