use openservicebroker as osb;

use actix_web::{App, HttpServer};
use actix_rt;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(
                osb::new_scope(
                    "",
                    osb::service::CatalogProvider::from_file_json("tests/default_catalog.json")
                )
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
