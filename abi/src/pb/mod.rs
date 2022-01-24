use prost::Message;
use prost_helper::{prost_into_vec, vec_try_into_prost};

mod abi_impl;
pub mod gen;

use bytes::Bytes;
use gen::*;

// Generate `From` trait for prost messages
prost_into_vec!((RequestPing, 64), (ResponsePong, 64));

// Generate `TryFrom` trait for Vec<u8>
vec_try_into_prost!(ResponsePong);

impl From<String> for Value {
    fn from(v: String) -> Self {
        Self {
            value: Some(value::Value::String(v)),
        }
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Self {
            value: Some(value::Value::String(v.into())),
        }
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Self {
            value: Some(value::Value::Integer(v)),
        }
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Self {
            value: Some(value::Value::Float(v)),
        }
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Self {
            value: Some(value::Value::Bool(v)),
        }
    }
}

impl From<Bytes> for Value {
    fn from(v: Bytes) -> Self {
        Self {
            value: Some(value::Value::Binary(v)),
        }
    }
}

impl Kvpair {
    pub fn new(key: impl Into<String>, value: Value) -> Self {
        Self {
            key: key.into(),
            val: Some(value),
        }
    }
}
