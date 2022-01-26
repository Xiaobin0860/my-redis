use crate::{Kvpair, Storage, StorageIter, Value};
use dashmap::{mapref::one::Ref, DashMap};

#[derive(Debug, Default)]
pub struct MemTable {
    tables: DashMap<String, DashMap<String, Value>>,
}

impl MemTable {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_or_create_table(&self, name: &str) -> Ref<String, DashMap<String, Value>> {
        match self.tables.get(name) {
            Some(table) => table,
            None => {
                let entry = self.tables.entry(name.into()).or_default();
                entry.downgrade()
            }
        }
    }
}

impl Storage for MemTable {
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, crate::KvError> {
        let tbl = self.get_or_create_table(table);
        Ok(tbl.get(key).map(|v| v.clone()))
    }

    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, crate::KvError> {
        let tbl = self.get_or_create_table(table);
        Ok(tbl.insert(key, value))
    }

    fn contains(&self, table: &str, key: &str) -> Result<bool, crate::KvError> {
        let tbl = self.get_or_create_table(table);
        Ok(tbl.contains_key(key))
    }

    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, crate::KvError> {
        let tbl = self.get_or_create_table(table);
        Ok(tbl.remove(key).map(|(_, v)| v))
    }

    fn get_all(&self, table: &str) -> Result<Vec<crate::Kvpair>, crate::KvError> {
        let tbl = self.get_or_create_table(table);
        Ok(tbl
            .iter()
            .map(|v| Kvpair::new(v.key(), v.value().clone()))
            .collect())
    }

    fn get_iter(
        &self,
        table: &str,
    ) -> Result<Box<dyn Iterator<Item = crate::Kvpair>>, crate::KvError> {
        let tbl = self.get_or_create_table(table).clone();
        Ok(Box::new(StorageIter::new(tbl.into_iter())))
    }
}
