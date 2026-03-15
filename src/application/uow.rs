use std::pin::Pin;

use sqlx::{Error, Postgres, Transaction, postgres::PgPool};

#[derive(Clone)]
pub(crate) struct UnitOfWork {
    pool: PgPool,
}

impl UnitOfWork {
    // note: PgPool is Clone and the clone remains tied to same connection pool
    pub async fn new() -> Result<Self, Error> {
        let pool = PgPool::connect("postgres://postgres:postgres@localhost/process").await?;

        Ok(Self { pool })
    }

    pub async fn execute<F>(&self, operation: F) -> Result<(), Error>
    where
        F: for<'c> FnOnce(
                &'c mut Transaction<'_, Postgres>,
            )
                -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'c>>
            + Send,
    {
        let mut tx: Transaction<'_, Postgres> = self.pool.begin().await?;

        // Execute the operation within the transaction
        let result = operation(&mut tx).await;

        match result {
            Ok(res) => {
                tx.commit().await?;
                Ok(res)
            }
            Err(e) => {
                tx.rollback().await?;
                Err(e)
            }
        }
    }
}
