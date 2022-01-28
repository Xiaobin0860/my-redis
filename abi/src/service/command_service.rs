use crate::*;

impl CommandService for Hget {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.get(&self.table, &self.key) {
            Ok(Some(v)) => v.into(),
            Ok(None) => KvError::NotFound(format!("table {}, key {}", self.table, self.key)).into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hgetall {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.get_all(&self.table) {
            Ok(v) => v.into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hmget {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        self.keys
            .iter()
            .map(|k| match store.get(&self.table, k) {
                Ok(Some(v)) => v,
                _ => Value::default(),
            })
            .collect::<Vec<_>>()
            .into()
    }
}

impl CommandService for Hset {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match self.pair {
            Some(p) => match store.set(&self.table, p.key, p.val.unwrap_or_default()) {
                Ok(Some(v)) => v.into(),
                Ok(None) => Value::default().into(),
                Err(e) => e.into(),
            },
            None => KvError::InvalidCommand(format!("{:?}", self)).into(),
        }
    }
}

impl CommandService for Hmset {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        self.pairs
            .into_iter()
            .map(
                |p| match store.set(&self.table, p.key, p.val.unwrap_or_default()) {
                    Ok(Some(v)) => v,
                    _ => Value::default(),
                },
            )
            .collect::<Vec<_>>()
            .into()
    }
}

impl CommandService for Hdel {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.del(&self.table, &self.key) {
            Ok(Some(v)) => v.into(),
            Ok(None) => Value::default().into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hmdel {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        self.keys
            .iter()
            .map(|k| match store.del(&self.table, k) {
                Ok(Some(v)) => v,
                _ => Value::default(),
            })
            .collect::<Vec<_>>()
            .into()
    }
}

impl CommandService for Hexist {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.contains(&self.table, &self.key) {
            Ok(v) => Value::from(v).into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hmexist {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        self.keys
            .iter()
            .map(|k| match store.contains(&self.table, k) {
                Ok(v) => v.into(),
                _ => Value::from(false),
            })
            .collect::<Vec<Value>>()
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hget_should_work() {
        let store = MemTable::new();
        let cmd = CommandRequest::new_hset("score", "u1", 10.into());
        dispatch(cmd, &store);
        let cmd = CommandRequest::new_hget("score", "u1");
        let res = dispatch(cmd, &store);
        assert_res_ok(&res, &[10.into()], &[]);

        let cmd = CommandRequest::new_hget("score", "no");
        let res = dispatch(cmd, &store);
        assert_res_error(&res, 404, "Not found");
    }

    #[test]
    fn hmget_should_work() {
        let store = MemTable::new();

        set_key_pairs(
            "user",
            vec![("u1", "Tyr"), ("u2", "Lindsey"), ("u3", "Rosie")],
            &store,
        );

        let cmd = CommandRequest::new_hmget("user", vec!["u1".into(), "u4".into(), "u3".into()]);
        let res = dispatch(cmd, &store);
        let values = &["Tyr".into(), Value::default(), "Rosie".into()];
        assert_res_ok(&res, values, &[]);
    }

    #[test]
    fn hgetall_should_work() {
        let store = MemTable::new();

        set_key_pairs(
            "score",
            vec![("u1", 10), ("u2", 8), ("u3", 11), ("u1", 6)],
            &store,
        );

        let cmd = CommandRequest::new_hgetall("score");
        let res = dispatch(cmd, &store);
        let pairs = &[
            Kvpair::new("u1", 6.into()),
            Kvpair::new("u2", 8.into()),
            Kvpair::new("u3", 11.into()),
        ];
        assert_res_ok(&res, &[], pairs);
    }

    #[test]
    fn hset_should_work() {
        let store = MemTable::new();
        let cmd = CommandRequest::new_hset("t1", "hello", "world".into());
        let res = dispatch(cmd.clone(), &store);
        assert_res_ok(&res, &[Value::default()], &[]);

        let res = dispatch(cmd, &store);
        assert_res_ok(&res, &["world".into()], &[]);
    }

    #[test]
    fn hmset_should_work() {
        let store = MemTable::new();
        set_key_pairs("t1", vec![("u1", "world")], &store);
        let pairs = vec![
            Kvpair::new("u1", 10.1.into()),
            Kvpair::new("u2", 8.1.into()),
        ];
        let cmd = CommandRequest::new_hmset("t1", pairs);
        let res = dispatch(cmd, &store);
        assert_res_ok(&res, &["world".into(), Value::default()], &[]);
    }

    #[test]
    fn hdel_should_work() {
        let store = MemTable::new();
        set_key_pairs("t1", vec![("u1", "v1")], &store);
        let cmd = CommandRequest::new_hdel("t1", "u2");
        let res = dispatch(cmd, &store);
        assert_res_ok(&res, &[Value::default()], &[]);

        let cmd = CommandRequest::new_hdel("t1", "u1");
        let res = dispatch(cmd, &store);
        assert_res_ok(&res, &["v1".into()], &[]);
    }

    #[test]
    fn hmdel_should_work() {
        let store = MemTable::new();
        set_key_pairs("t1", vec![("u1", "v1"), ("u2", "v2")], &store);

        let cmd = CommandRequest::new_hmdel("t1", vec!["u1".into(), "u3".into()]);
        let res = dispatch(cmd, &store);
        assert_res_ok(&res, &["v1".into(), Value::default()], &[]);
    }

    #[test]
    fn hexist_should_work() {
        let store = MemTable::new();
        set_key_pairs("t1", vec![("u1", "v1")], &store);
        let cmd = CommandRequest::new_hexist("t1", "u2");
        let res = dispatch(cmd, &store);
        assert_res_ok(&res, &[false.into()], &[]);

        let cmd = CommandRequest::new_hexist("t1", "u1");
        let res = dispatch(cmd, &store);
        assert_res_ok(&res, &[true.into()], &[]);
    }

    #[test]
    fn hmexists_should_work() {
        let store = MemTable::new();
        set_key_pairs("t1", vec![("u1", "v1"), ("u2", "v2")], &store);

        let cmd = CommandRequest::new_hmexist("t1", vec!["u1".into(), "u3".into()]);
        let res = dispatch(cmd, &store);
        assert_res_ok(&res, &[true.into(), false.into()], &[]);
    }

    fn set_key_pairs<T>(table: &str, pairs: Vec<(&str, T)>, store: &impl Storage)
    where
        T: Into<Value>,
    {
        pairs
            .into_iter()
            .map(|(key, value)| CommandRequest::new_hset(table, key, value.into()))
            .for_each(|cmd| {
                dispatch(cmd, store);
            });
    }
}
