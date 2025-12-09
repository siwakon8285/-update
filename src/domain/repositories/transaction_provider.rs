use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TransactionProvider {
    async fn transaction<T, E, F>(&self, f: F) -> Result<T, E>
    where
        F: FnOnce(&mut diesel::PgConnection) -> Result<T, E> + Send + 'static,
        T: Send + 'static,
        E: Send + 'static + From<anyhow::Error>;
}
