use std::{ io };

use actix_web::{ HttpServer, App };

mod route;
mod libs;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(route::html::index)
            .service(route::html::login)
            .service(route::api::login)
            .service(route::api::get_profile)
            .service(route::api::logout)
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}