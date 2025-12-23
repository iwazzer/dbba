use crate::adapters::DatabaseAdapter;
use crate::error::{DbbaError, Result};
use crate::models::{DatabaseSnapshot, DbInfo};
use async_trait::async_trait;
use chrono::NaiveDateTime;
use md5::{Digest, Md5};
use serde_json::Value;
use sqlx::mysql::{MySqlPool, MySqlRow};
use sqlx::{Column, Row, TypeInfo, ValueRef};
use std::collections::{BTreeMap, HashMap};

/// MySQL database adapter
pub struct MySqlAdapter {
    db_info: DbInfo,
    pool: Option<MySqlPool>,
}

impl MySqlAdapter {
    pub fn new(db_info: DbInfo) -> Self {
        Self {
            db_info,
            pool: None,
        }
    }

    /// Get the connection pool
    fn pool(&self) -> Result<&MySqlPool> {
        self.pool
            .as_ref()
            .ok_or_else(|| DbbaError::Config("Database not connected".to_string()))
    }

    /// Convert a MySQL row to a BTreeMap (sorted keys)
    fn row_to_hashmap(row: &MySqlRow) -> Result<BTreeMap<String, Value>> {
        let mut map = BTreeMap::new();

        for (i, column) in row.columns().iter().enumerate() {
            let column_name = column.name();
            let type_info = column.type_info();
            let type_name = type_info.name();

            // Check if the value is NULL first
            let raw_value = row.try_get_raw(i)?;
            if raw_value.is_null() {
                map.insert(column_name.to_string(), Value::Null);
                continue;
            }

            // Handle different MySQL types based on type name
            let value: Value = match type_name {
                "TINYINT" | "SMALLINT" | "MEDIUMINT" | "INT" | "BIGINT" => {
                    row.try_get::<i64, _>(i)
                        .map(|v| Value::Number(v.into()))
                        .unwrap_or(Value::Null)
                }
                "FLOAT" | "DOUBLE" | "DECIMAL" => {
                    row.try_get::<f64, _>(i)
                        .ok()
                        .and_then(|v| serde_json::Number::from_f64(v))
                        .map(Value::Number)
                        .unwrap_or(Value::Null)
                }
                "DATETIME" | "TIMESTAMP" => {
                    row.try_get::<NaiveDateTime, _>(i)
                        .map(|v| Value::String(v.format("%Y-%m-%d %H:%M:%S").to_string()))
                        .unwrap_or(Value::Null)
                }
                "BLOB" | "TINYBLOB" | "MEDIUMBLOB" | "LONGBLOB" | "BINARY" | "VARBINARY" => {
                    row.try_get::<Vec<u8>, _>(i)
                        .map(|v| {
                            let mut hasher = Md5::new();
                            hasher.update(&v);
                            let hash = hasher.finalize();
                            Value::String(format!("MD5 Digest value: {:x}", hash))
                        })
                        .unwrap_or(Value::Null)
                }
                "BIT" => {
                    row.try_get::<bool, _>(i)
                        .map(Value::Bool)
                        .unwrap_or(Value::Null)
                }
                // Try Vec<u8> first for unknown types (might be binary), then String
                _ => {
                    if let Ok(v) = row.try_get::<Vec<u8>, _>(i) {
                        let mut hasher = Md5::new();
                        hasher.update(&v);
                        let hash = hasher.finalize();
                        Value::String(format!("MD5 Digest value: {:x}", hash))
                    } else if let Ok(v) = row.try_get::<String, _>(i) {
                        Value::String(v)
                    } else {
                        Value::Null
                    }
                }
            };

            map.insert(column_name.to_string(), value);
        }

        Ok(map)
    }
}

#[async_trait]
impl DatabaseAdapter for MySqlAdapter {
    async fn connect(&mut self) -> Result<()> {
        let pool = MySqlPool::connect(&self.db_info.connection_url()).await?;
        self.pool = Some(pool);
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<()> {
        if let Some(pool) = self.pool.take() {
            pool.close().await;
        }
        Ok(())
    }

    async fn read_database(&self) -> Result<DatabaseSnapshot> {
        let pool = self.pool()?;
        let tables = self.list_tables().await?;

        let mut snapshot = HashMap::new();

        for table_name in tables {
            // Query all data from the table
            let query = format!("SELECT * FROM `{}`", table_name);
            let rows = sqlx::query(&query).fetch_all(pool).await?;

            let mut table_data = Vec::new();
            for row in rows.iter() {
                let row_map = Self::row_to_hashmap(row)?;
                table_data.push(row_map);
            }

            snapshot.insert(table_name, table_data);
        }

        Ok(snapshot)
    }

    async fn list_tables(&self) -> Result<Vec<String>> {
        let pool = self.pool()?;
        let query = "SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES WHERE table_schema = ?";

        let rows = sqlx::query(query)
            .bind(&self.db_info.database)
            .fetch_all(pool)
            .await?;

        let mut table_names = Vec::new();
        for row in rows {
            // Get TABLE_NAME as either String or Vec<u8>
            let name = if let Ok(n) = row.try_get::<String, _>(0) {
                n
            } else if let Ok(n) = row.try_get::<Vec<u8>, _>(0) {
                String::from_utf8_lossy(&n).to_string()
            } else {
                continue;
            };
            table_names.push(name);
        }

        Ok(table_names)
    }
}
