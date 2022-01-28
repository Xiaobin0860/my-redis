use http::StatusCode;
use prost::Message;
use prost_helper::{prost_into_vec, vec_try_into_prost};

mod abi_impl;
pub mod gen;

use bytes::Bytes;
use gen::{command_request::Request, *};

use crate::KvError;

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

impl From<(String, Value)> for Kvpair {
    fn from(v: (String, Value)) -> Self {
        Kvpair::new(v.0, v.1)
    }
}

impl From<KvError> for CommandResponse {
    fn from(e: KvError) -> Self {
        let status = match e {
            KvError::InvalidCommand(_) => StatusCode::BAD_REQUEST,
            KvError::NotFound(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
        .as_u16() as u32;
        Self {
            status,
            message: e.to_string(),
            ..Default::default()
        }
    }
}

impl From<Value> for CommandResponse {
    fn from(v: Value) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            values: vec![v],
            ..Default::default()
        }
    }
}

impl From<Vec<Value>> for CommandResponse {
    fn from(v: Vec<Value>) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            values: v,
            ..Default::default()
        }
    }
}

impl From<Vec<Kvpair>> for CommandResponse {
    fn from(v: Vec<Kvpair>) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            pairs: v,
            ..Default::default()
        }
    }
}

impl CommandRequest {
    pub fn new_hget(table: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            request: Some(Request::Hget(Hget {
                table: table.into(),
                key: key.into(),
            })),
        }
    }

    pub fn new_hgetall(table: impl Into<String>) -> Self {
        Self {
            request: Some(Request::Hgetall(Hgetall {
                table: table.into(),
            })),
        }
    }

    pub fn new_hmget(table: impl Into<String>, keys: Vec<String>) -> Self {
        Self {
            request: Some(Request::Hmget(Hmget {
                table: table.into(),
                keys,
            })),
        }
    }

    pub fn new_hset(table: impl Into<String>, key: impl Into<String>, value: Value) -> Self {
        Self {
            request: Some(Request::Hset(Hset {
                table: table.into(),
                pair: Some(Kvpair::new(key, value)),
            })),
        }
    }

    pub fn new_hmset(table: impl Into<String>, pairs: Vec<Kvpair>) -> Self {
        Self {
            request: Some(Request::Hmset(Hmset {
                table: table.into(),
                pairs,
            })),
        }
    }

    pub fn new_hdel(table: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            request: Some(Request::Hdel(Hdel {
                table: table.into(),
                key: key.into(),
            })),
        }
    }

    pub fn new_hmdel(table: impl Into<String>, keys: Vec<String>) -> Self {
        Self {
            request: Some(Request::Hmdel(Hmdel {
                table: table.into(),
                keys,
            })),
        }
    }

    pub fn new_hexist(table: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            request: Some(Request::Hexist(Hexist {
                table: table.into(),
                key: key.into(),
            })),
        }
    }

    pub fn new_hmexist(table: impl Into<String>, keys: Vec<String>) -> Self {
        Self {
            request: Some(Request::Hmexist(Hmexist {
                table: table.into(),
                keys,
            })),
        }
    }

    /// 转换成 string 做错误处理
    pub fn format(&self) -> String {
        format!("{:?}", self)
    }
}

impl CommandResponse {
    pub fn ok() -> Self {
        CommandResponse {
            status: StatusCode::OK.as_u16() as _,
            ..Default::default()
        }
    }

    pub fn internal_error(msg: String) -> Self {
        CommandResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16() as _,
            message: msg,
            ..Default::default()
        }
    }

    /// 转换成 string 做错误处理
    pub fn format(&self) -> String {
        format!("{:?}", self)
    }
}
