use crate::error::Result;
use crate::models::DatabaseSnapshot;
use async_trait::async_trait;

/// Trait for database adapters that can read database snapshots
#[async_trait]
pub trait DatabaseAdapter: Send + Sync {
    /// Connect to the database
    async fn connect(&mut self) -> Result<()>;

    /// Disconnect from the database
    async fn disconnect(&mut self) -> Result<()>;

    /// Read all tables and their data from the database
    async fn read_database(&self) -> Result<DatabaseSnapshot>;

    /// List all table names in the database
    async fn list_tables(&self) -> Result<Vec<String>>;
}
