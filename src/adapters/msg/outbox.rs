use sqlx::{Error, Transaction, postgres::Postgres};
use super::Message;

#[derive(Clone)]
pub(crate) struct Outbox {}
impl Outbox {
    pub async fn post(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        message: Message,
    ) -> Result<(), Error> {
        let id = message.id();
        let msg = message.msg();
        sqlx::query("INSERT INTO outbox (id, msg) VALUES ($1, $2)")
            .bind(&id)
            .bind(msg)
            .execute(&mut **tx)
            .await?;

        Ok(())
    }
}
