use actix_web::{web, App, HttpRequest, HttpServer, Responder, get, post};

#[get("/")]
fn home(req: HttpRequest) -> impl Responder {
    let word = req.match_info().get("word").unwrap_or("Blockchain");
    format!("Hello {}!", &word)
}

#[get("/blocks")]
fn blocks(req: HttpRequest) -> impl Responder {
    let word = req.match_info().get("word").unwrap_or("Blockchain");
    format!("Hello {}!", &word)
}

#[post("/mine-block")]
fn mine_block(req: HttpRequest) -> impl Responder {
    let word = req.match_info().get("word").unwrap_or("Blockchain");
    format!("Hello {}!", &word)
}

#[get("/peers")]
fn peers(req: HttpRequest) -> impl Responder {
    let word = req.match_info().get("word").unwrap_or("Blockchain");
    format!("Hello {}!", &word)
}

#[post("/add-peer")]
fn add_peer(req: HttpRequest) -> impl Responder {
    let word = req.match_info().get("word").unwrap_or("Blockchain");
    format!("Hello {}!", &word)
}

pub fn start() {
    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(blocks)
            .service(mine_block)
            .service(peers)
            .service(add_peer)
    })
        .bind("127.0.0.1:1461")
        .expect("Can not bind to port 1461")
        .run()
        .unwrap();
}