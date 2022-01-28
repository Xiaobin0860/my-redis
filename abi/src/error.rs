use thiserror::Error;

use crate::*;
use macros::{respone_to_result, result_to_vec};

/// General error definition for the project
#[derive(Error, Debug)]
pub enum KvError {
    // detailed errors
    #[error("Unsupported API: {0}")]
    UnsupportedApi(String),
    #[error("Malformed API response for {0}")]
    MalformedApiResponse(String),
    #[error("Command is invalid: `{0}`")]
    InvalidCommand(String),
    #[error("Not found: {0}")]
    NotFound(String),

    // converted errors
    #[error("Protobuf decode error: {0}")]
    ProstDecodeError(#[from] prost::DecodeError),
    #[error("Protobuf decode error: {0}")]
    ProstEncodeError(#[from] prost::EncodeError),
}

impl From<KvError> for AppError {
    fn from(err: KvError) -> Self {
        AppError::new(get_code(&err), err.to_string())
    }
}

impl From<&AppError> for KvError {
    fn from(err: &AppError) -> Self {
        match AppErrorCode::from_i32(err.code).unwrap() {
            AppErrorCode::UnsupportedApi => KvError::UnsupportedApi(err.message.to_owned()),
            AppErrorCode::MalformedApiResponse => {
                KvError::MalformedApiResponse(err.message.to_owned())
            }

            // converted errors
            _ => unimplemented!(),
        }
    }
}

fn get_code(e: &KvError) -> AppErrorCode {
    match e {
        KvError::UnsupportedApi(_) => AppErrorCode::UnsupportedApi,
        KvError::MalformedApiResponse(_) => AppErrorCode::MalformedApiResponse,
        KvError::InvalidCommand(_) => AppErrorCode::InvalidCommand,
        KvError::NotFound(_) => AppErrorCode::NotFound,

        // converted errors
        KvError::ProstDecodeError(_) => AppErrorCode::ProstDecodeError,
        KvError::ProstEncodeError(_) => AppErrorCode::ProstEncodeError,
    }
}

/// convert protobuf type ResponseMsg into a Result<&Msg, &KvError>
pub trait ToResult {
    /// internal type for the ResponseMsg
    type Msg;

    /// extract Msg or AppError to Result
    fn to_result(&self) -> Result<&Self::Msg, KvError>;
}

/// Convert Result<Msg, KvError> into protobuf bytes
pub trait ToVec {
    /// generate protobuf bytes based on Result
    fn to_vec(&self) -> Vec<u8>;
}

respone_to_result!(ResponsePong, Pong);
result_to_vec!(ResponsePong, Pong);
