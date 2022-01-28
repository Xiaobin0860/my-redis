use crate::{command_request::Request, CommandRequest, CommandResponse, KvError, Storage};

mod command_service;

///对Command处理的抽象
pub trait CommandService {
    fn execute(self, store: &impl Storage) -> CommandResponse;
}

///从Request中得到Response
pub fn dispatch(cmd: CommandRequest, store: &impl Storage) -> CommandResponse {
    match cmd.request {
        Some(Request::Hget(param)) => param.execute(store),
        Some(Request::Hgetall(param)) => param.execute(store),
        Some(Request::Hmget(param)) => param.execute(store),
        Some(Request::Hset(param)) => param.execute(store),
        Some(Request::Hmset(param)) => param.execute(store),
        Some(Request::Hdel(param)) => param.execute(store),
        Some(Request::Hmdel(param)) => param.execute(store),
        Some(Request::Hexist(param)) => param.execute(store),
        Some(Request::Hmexist(param)) => param.execute(store),
        None => KvError::InvalidCommand("Request has no request".into()).into(),
    }
}

#[cfg(test)]
use crate::{Kvpair, Value};

// 测试成功返回的结果
#[cfg(test)]
pub fn assert_res_ok(res: &CommandResponse, values: &[Value], pairs: &[Kvpair]) {
    let mut sorted_pairs = res.pairs.clone();
    sorted_pairs.sort_by(|a, b| a.key.partial_cmp(&b.key).unwrap());
    assert_eq!(res.status, 200);
    assert_eq!(res.message, "");
    assert_eq!(res.values, values);
    assert_eq!(sorted_pairs, pairs);
}

// 测试失败返回的结果
#[cfg(test)]
pub fn assert_res_error(res: &CommandResponse, code: u32, msg: &str) {
    assert_eq!(res.status, code);
    assert!(res.message.contains(msg));
    assert_eq!(res.values, &[]);
    assert_eq!(res.pairs, &[]);
}
