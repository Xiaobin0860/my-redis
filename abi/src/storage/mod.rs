mod memory;

pub use memory::MemTable;

use crate::{KvError, Kvpair, Value};

/// 存储接口
pub trait Storage {
    ///获取一个key的value
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    ///设置一个key的value，返回旧的value
    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError>;
    ///是否存在key
    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError>;
    ///删除一个key
    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    ///获取所有kv
    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError>;
    ///获取遍历kv的Iterator
    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError>;
}

/// 提供 Storage iterator，这样 trait 的实现者只需要
/// 把它们的 iterator 提供给 StorageIter，然后它们保证
/// next() 传出的类型实现了 Into<Kvpair> 即可
pub struct StorageIter<T> {
    data: T,
}

impl<T> StorageIter<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl<T> Iterator for StorageIter<T>
where
    T: Iterator,
    T::Item: Into<Kvpair>,
{
    type Item = Kvpair;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.next().map(|v| v.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memtable_basic_interface_should_work() {
        let store = MemTable::new();
        test_basic_interface(store);
    }

    #[test]
    fn memtable_get_all_should_work() {
        let store = MemTable::new();
        test_get_all(store);
    }

    #[test]
    fn memtable_get_iter_should_work() {
        let store = MemTable::new();
        test_get_iter(store);
    }

    fn test_basic_interface(store: impl Storage) {
        //首次set返回None
        let v = store.set("t1", "k1".into(), "v1".into()).unwrap();
        assert!(v.is_none());
        let v = store.get("t1", "k1").unwrap();
        assert_eq!(Some("v1".into()), v);
        //再次set返回之前的值
        let v = store.set("t1", "k1".into(), "v".into()).unwrap();
        assert_eq!(Some("v1".into()), v);
        let v = store.get("t1", "k1").unwrap();
        assert_eq!(Some("v".into()), v);
        //取不存在的key或表返回None
        assert!(store.get("t1", "no").unwrap().is_none());
        assert!(store.get("t2", "k1").unwrap().is_none());
        //contains存在的key返回true，否则false
        assert!(store.contains("t1", "k1").unwrap());
        assert!(!store.contains("t1", "no").unwrap());
        assert!(!store.contains("t2", "k1").unwrap());
        //删存在的key返回之前的值
        let v = store.del("t1", "k1").unwrap();
        assert_eq!(Some("v".into()), v);
        //删不存在的key或表返回None
        assert!(store.del("t1", "k1").unwrap().is_none());
        assert!(store.del("t2", "k1").unwrap().is_none());
    }

    fn test_get_all(store: impl Storage) {
        store.set("t1", "k1".into(), "v1".into()).unwrap();
        store.set("t1", "k2".into(), "v2".into()).unwrap();
        let mut vs = store.get_all("t1").unwrap();
        vs.sort_by(|a, b| a.key.partial_cmp(&b.key).unwrap());
        assert_eq!(
            vec![
                Kvpair::new("k1", "v1".into()),
                Kvpair::new("k2", "v2".into())
            ],
            vs
        );
    }

    fn test_get_iter(store: impl Storage) {
        store.set("t1", "k1".into(), "v1".into()).unwrap();
        store.set("t1", "k2".into(), "v2".into()).unwrap();
        let mut vs: Vec<_> = store.get_iter("t1").unwrap().collect();
        vs.sort_by(|a, b| a.key.partial_cmp(&b.key).unwrap());
        assert_eq!(
            vec![
                Kvpair::new("k1", "v1".into()),
                Kvpair::new("k2", "v2".into())
            ],
            vs
        );
    }
}
