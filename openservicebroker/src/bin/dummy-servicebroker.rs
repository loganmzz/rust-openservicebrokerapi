use openservicebroker as osb;

use actix_web::{web, App, HttpServer};
use actix_rt;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/v2/catalog", web::get().to(osb::get_catalog))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
