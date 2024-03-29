use actix_files::Files;
use actix_web_actors::ws;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};

mod message;
mod server;
mod session;
mod room_logic;

async fn client_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(session::PokerSesssion::default(), &req, stream)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let addr = "0.0.0.0:1234";
    HttpServer::new(move || {
        App::new()
            .service(web::resource("/ws").to(client_route))
            .service(Files::new("/", "./static/").index_file("index.html"))
    })
    .bind(&addr)?
    .run()
    .await
}
