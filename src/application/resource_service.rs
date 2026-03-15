use sqlx::Error;

use crate::{adapters::{Outbox, ResourceRepo}, application::UnitOfWork, domain::Resource};

use super::create_uow::CreateResourceUoW;

pub struct ResourceService {
    create: CreateResourceUoW,
}

impl ResourceService {
    pub fn new(resource_repo: ResourceRepo, outbox: Outbox, uow: UnitOfWork) -> Self {
        let create = CreateResourceUoW::new(uow, resource_repo, outbox);
        Self { create }
    }

    pub async fn create_resource(&self, id: i64, name: String) -> Result<Resource, Error> {
        self.create.create_resource(id, name).await
    }
}