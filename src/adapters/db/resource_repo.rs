use crate::domain::Resource;
use sqlx::{Error, Transaction, postgres::Postgres};

#[derive(Clone)]
pub(crate) struct ResourceRepo {}
impl ResourceRepo {
    pub async fn save(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        resource: &Resource,
    ) -> Result<(), Error> {
        let id = resource.id();
        let name = resource.name();
        sqlx::query("INSERT INTO resources (id, name) VALUES ($1, $2)")
            .bind(&id)
            .bind(name)
            .execute(&mut **tx)
            .await?;

        Ok(())
    }
}
