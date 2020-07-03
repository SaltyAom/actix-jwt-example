use std::io;
use dotenv::dotenv;

use actix_web::{ HttpServer, App, middleware };

mod route;
mod libs;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .wrap(
                middleware::Compress::default()
            )
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