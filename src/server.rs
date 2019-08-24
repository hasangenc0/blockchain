use actix_web::{web, App, HttpRequest, HttpServer, Responder};

fn test(req: HttpRequest) -> impl Responder {
    let word = req.match_info().get("word").unwrap_or("Blockchain");
    format!("Hello {}!", &word)
}

pub fn start() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(test))
            .route("/{word}", web::get().to(test))
            .route("/blocks", web::get().to(test))
            .route("/mine-block", web::post().to(test))
            .route("/peers", web::get().to(test))
            .route("/add-peer", web::post().to(test))
    })
        .bind("127.0.0.1:1461")
        .expect("Can not bind to port 1461")
        .run()
        .unwrap();
}