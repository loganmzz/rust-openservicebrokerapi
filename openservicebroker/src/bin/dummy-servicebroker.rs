use openservicebroker as osb;

use actix_web::{App, HttpServer};
use actix_rt;

use anyhow::Result;
use anyhow::Context;

#[actix_rt::main]
async fn main() -> Result<()> {
    let catalog = osb::service::CatalogProvider::from_file_json("tests/default_catalog.json")
                                                .with_context(|| "Error on loading default catalog")?;
    HttpServer::new(move || {
        App::new()
            .service(osb::new_scope("", catalog.clone()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;
    Ok(())
}
