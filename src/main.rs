#![feature(async_await)]
mod compat;

use actix_web::web::Bytes;
use actix_web::HttpRequest;
use actix_web::{web, App, Error, HttpResponse, HttpServer};
use compat::into_01;

/// extract path info using serde
async fn index(req: HttpRequest, body: Bytes) -> Result<HttpResponse, Error> {
    dbg!(req);
    dbg!(body);
    Ok(HttpResponse::Ok().body("hello world"))
}

fn main() {
    HttpServer::new(|| {
        App::new().service(
            web::resource("/")
                .route(web::route().to_async(|arg1, arg2| into_01(index(arg1, arg2)))),
        )
    })
    .bind("127.0.0.1:6000")
    .unwrap()
    .run()
    .unwrap();
}
