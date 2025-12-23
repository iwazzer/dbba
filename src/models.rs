use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

/// Database connection information
#[derive(Debug, Clone)]
pub struct DbInfo {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    pub encoding: String,
}

impl DbInfo {
    pub fn new(
        host: String,
        port: u16,
        username: String,
        password: String,
        database: String,
        encoding: String,
    ) -> Self {
        Self {
            host,
            port,
            username,
            password,
            database,
            encoding,
        }
    }

    /// Build MySQL connection URL
    pub fn connection_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}?charset={}",
            self.username, self.password, self.host, self.port, self.database, self.encoding
        )
    }
}

/// Represents data from a single table (using BTreeMap for sorted keys)
pub type TableData = Vec<BTreeMap<String, serde_json::Value>>;

/// Represents the entire database snapshot (table_name -> records)
pub type DatabaseSnapshot = HashMap<String, TableData>;

/// Represents changes detected in a table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableChanges {
    pub table_name: String,
    pub deleted_ids: Vec<String>,
    pub added_ids: Vec<String>,
    pub modified_ids: Vec<String>,
}

impl TableChanges {
    pub fn has_changes(&self) -> bool {
        !self.deleted_ids.is_empty() || !self.added_ids.is_empty() || !self.modified_ids.is_empty()
    }
}

/// Represents all changes in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseChanges {
    pub tables: Vec<TableChanges>,
}

impl DatabaseChanges {
    pub fn new() -> Self {
        Self { tables: Vec::new() }
    }

    pub fn has_changes(&self) -> bool {
        self.tables.iter().any(|t| t.has_changes())
    }
}

impl Default for DatabaseChanges {
    fn default() -> Self {
        Self::new()
    }
}
