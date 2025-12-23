use crate::adapters::{DatabaseAdapter, OutputAdapter};
use crate::error::Result;
use crate::models::TableChanges;
use std::collections::{BTreeMap, HashMap, HashSet};

/// Core database diff logic
pub struct DbDiff<D: DatabaseAdapter, O: OutputAdapter> {
    db_adapter: D,
    output_adapter: O,
}

impl<D: DatabaseAdapter, O: OutputAdapter> DbDiff<D, O> {
    /// Create a new DbDiff instance
    pub fn new(db_adapter: D, output_adapter: O) -> Self {
        Self {
            db_adapter,
            output_adapter,
        }
    }

    /// Execute the database diff process
    pub async fn execute(&mut self) -> Result<bool> {
        println!("now reading db...");
        let before_db = self.db_adapter.read_database().await?;

        println!("run usecase now. then press any key when done.");
        Self::wait_for_input()?;

        println!("now reading db...");
        let after_db = self.db_adapter.read_database().await?;

        self.output_adapter.start_output()?;

        let mut has_changes = false;

        // Sort table names for consistent output order
        let mut table_names: Vec<_> = before_db.keys().collect();
        table_names.sort();

        for table_name in table_names {
            let before_records = before_db.get(table_name).unwrap();
            let after_records = after_db.get(table_name).cloned().unwrap_or_default();

            // Convert records to JSON strings indexed by ID
            let before_map = Self::records_to_json_map(before_records);
            let after_map = Self::records_to_json_map(&after_records);

            // Detect changes
            let changes = detect_changes(&before_map, &after_map);

            if changes.has_changes() {
                has_changes = true;

                self.output_adapter.write_title(table_name)?;

                // Get all IDs that have changes
                let all_ids: HashSet<String> = changes
                    .deleted_ids
                    .iter()
                    .chain(changes.added_ids.iter())
                    .chain(changes.modified_ids.iter())
                    .cloned()
                    .collect();

                for id in all_ids {
                    let left = before_map.get(&id).map(|s| s.as_str()).unwrap_or("");
                    let right = after_map.get(&id).map(|s| s.as_str()).unwrap_or("");

                    let (left_html, right_html) = self.output_adapter.generate_diff(left, right);
                    self.output_adapter
                        .write_diff_section(&left_html, &right_html)?;
                }

                self.output_adapter.close_section()?;
            }
        }

        if !has_changes {
            self.output_adapter.write_no_diff_message()?;
        }

        self.output_adapter.end_output()?;
        println!("done.");

        Ok(has_changes)
    }

    /// Convert records to a HashMap of ID -> JSON string
    fn records_to_json_map(
        records: &[BTreeMap<String, serde_json::Value>],
    ) -> HashMap<String, String> {
        records
            .iter()
            .filter_map(|record| {
                let id = record
                    .get("id")
                    .and_then(|v| match v {
                        serde_json::Value::Number(n) => Some(n.to_string()),
                        serde_json::Value::String(s) => Some(s.clone()),
                        _ => None,
                    })
                    .unwrap_or_else(|| "unknown".to_string());

                serde_json::to_string_pretty(record)
                    .ok()
                    .map(|json| (id, json))
            })
            .collect()
    }

    /// Wait for user input
    fn wait_for_input() -> Result<()> {
        use std::io::{stdin, BufRead};
        let stdin = stdin();
        let mut lines = stdin.lock().lines();
        lines.next();
        Ok(())
    }
}

/// Detect changes between before and after maps
fn detect_changes(
    before: &HashMap<String, String>,
    after: &HashMap<String, String>,
) -> TableChanges {
    let before_keys: HashSet<_> = before.keys().cloned().collect();
    let after_keys: HashSet<_> = after.keys().cloned().collect();

    let deleted_ids: Vec<String> = before_keys.difference(&after_keys).cloned().collect();
    let added_ids: Vec<String> = after_keys.difference(&before_keys).cloned().collect();
    let modified_ids: Vec<String> = before_keys
        .intersection(&after_keys)
        .filter(|id| before.get(*id) != after.get(*id))
        .cloned()
        .collect();

    TableChanges {
        table_name: String::new(),
        deleted_ids,
        added_ids,
        modified_ids,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_changes_no_changes() {
        let mut before = HashMap::new();
        before.insert("1".to_string(), r#"{"id":1,"name":"John"}"#.to_string());

        let mut after = HashMap::new();
        after.insert("1".to_string(), r#"{"id":1,"name":"John"}"#.to_string());

        let changes = detect_changes(&before, &after);

        assert_eq!(changes.deleted_ids.len(), 0);
        assert_eq!(changes.added_ids.len(), 0);
        assert_eq!(changes.modified_ids.len(), 0);
        assert!(!changes.has_changes());
    }

    #[test]
    fn test_detect_changes_with_additions() {
        let before = HashMap::new();

        let mut after = HashMap::new();
        after.insert("1".to_string(), r#"{"id":1,"name":"John"}"#.to_string());

        let changes = detect_changes(&before, &after);

        assert_eq!(changes.added_ids.len(), 1);
        assert!(changes.added_ids.contains(&"1".to_string()));
        assert!(changes.has_changes());
    }

    #[test]
    fn test_detect_changes_with_deletions() {
        let mut before = HashMap::new();
        before.insert("1".to_string(), r#"{"id":1,"name":"John"}"#.to_string());

        let after = HashMap::new();

        let changes = detect_changes(&before, &after);

        assert_eq!(changes.deleted_ids.len(), 1);
        assert!(changes.deleted_ids.contains(&"1".to_string()));
        assert!(changes.has_changes());
    }

    #[test]
    fn test_detect_changes_with_modifications() {
        let mut before = HashMap::new();
        before.insert("1".to_string(), r#"{"id":1,"name":"John"}"#.to_string());

        let mut after = HashMap::new();
        after.insert("1".to_string(), r#"{"id":1,"name":"Jane"}"#.to_string());

        let changes = detect_changes(&before, &after);

        assert_eq!(changes.modified_ids.len(), 1);
        assert!(changes.modified_ids.contains(&"1".to_string()));
        assert!(changes.has_changes());
    }
}
