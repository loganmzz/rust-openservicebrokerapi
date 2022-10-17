use openservicebroker as osb;
use osb::service::CatalogProvider;

use actix_web::{App, HttpServer};

use anyhow::Result;
use anyhow::Context;

#[actix_web::main]
async fn main() -> Result<()> {
    let catalog = osb::service::providers::catalog::file_json("tests/default_catalog.json")
                                                   .to_single()
                                                   .with_context(|| "Error on loading default catalog")?;
    HttpServer::new(move || {
        App::new()
            .service(osb::new_scope("", Box::new(catalog.clone())))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;
    Ok(())
}
