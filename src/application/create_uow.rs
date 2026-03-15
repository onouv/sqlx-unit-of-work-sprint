use std::pin::Pin;
use std::sync::Arc;

use sqlx::Error;

use super::uow::UnitOfWork;
use crate::adapters::Outbox;
use crate::adapters::ResourceRepo;
use crate::domain::Resource;

pub struct CreateResourceUoW {
    uow: UnitOfWork,
    repo: Arc<ResourceRepo>,
    outbox: Arc<Outbox>,
}

impl CreateResourceUoW {
    pub fn new(uow: UnitOfWork, repo: ResourceRepo, outbox: Outbox) -> Self {
        Self {
            uow,
            repo: Arc::new(repo),
            outbox: Arc::new(outbox),
        }
    }

    pub async fn create_resource(&self, id: i64, name: String) -> Result<Resource, Error> {
        let repo = Arc::clone(&self.repo);
        let outbox = Arc::clone(&self.outbox);
        let resource_name = name.clone();

        self.uow
            .execute(|tx| {
                Box::pin(async move {
                    let resource = Resource::new(id, &name);
                    repo.save(tx, &resource).await?;
                    outbox.post(tx, resource.into()).await?;

                    Ok(())
                })
            })
            .await?;

        Ok(Resource::new(id, &resource_name))
    }
}
