mod adapters;
mod application;
mod domain;
use application::ResourceService;
use sqlx::Error;

use crate::{
    adapters::{Outbox, ResourceRepo},
    application::UnitOfWork,
};

#[tokio::main]
async fn main() -> Result<(), Error> {

    let service = setup_application_layer().await;

    let res1 = service.create_resource(1, "resource 1".to_string()).await?;
    println!("{}", res1);

    Ok(())
}

async fn setup_application_layer() -> ResourceService {
    let uow = UnitOfWork::new().await.unwrap();

    ResourceService::new(ResourceRepo {}, Outbox {}, uow)
}