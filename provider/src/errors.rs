/*
 * Copyright 2020 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::{Result, GetStatusServiceResult, Status, ProviderServiceResult, SUCCESS_CODE};

use fce_sqlite_connector::Error as SqliteConnectorError;
use fce_sqlite_connector::Value;

use std::convert::From;
use std::error::Error;

#[derive(Debug)]
pub enum ProviderError {
    SqliteConnectorError(SqliteConnectorError),
    CorruptedMessage(Vec<Value>),
    InternalError(String),
    InvalidArgument(String),
    UnexpectedValueType(Value, &'static str),
}

impl Error for ProviderError {}

impl std::fmt::Display for ProviderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::SqliteConnectorError(err) => writeln!(f, "{:?}", err),
            Self::CorruptedMessage(values) => writeln!(
                f,
                "message can't be constructed from returned values: {:?}",
                values
            ),
            Self::InternalError(err_msg) => writeln!(f, "{}", err_msg),
            Self::InvalidArgument(err_msg) => writeln!(f, "{}", err_msg),
            Self::UnexpectedValueType(value, expected_type) => writeln!(
                f,
                "expected type {}, but value {:?} received",
                expected_type, value
            ),
        }
    }
}

impl From<SqliteConnectorError> for ProviderError {
    fn from(err: SqliteConnectorError) -> Self {
        ProviderError::SqliteConnectorError(err)
    }
}

impl From<std::convert::Infallible> for ProviderError {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
    }
}

fn to_error_core(err: &ProviderError) -> i32 {
    match err {
        ProviderError::SqliteConnectorError(_) => 0,
        ProviderError::CorruptedMessage(_) => 1,
        ProviderError::InternalError(_) => 2,
        ProviderError::InvalidArgument(_) => 3,
        ProviderError::UnexpectedValueType(..) => 4,
    }
}

impl From<Result<bool>> for GetStatusServiceResult {
    fn from(result: Result<bool>) -> Self {
        match result {
            Ok(s) => Self {
                ret_code: SUCCESS_CODE,
                err_msg: String::new(),
                status: Status {is_registered: s},
            },
            Err(err) => Self {
                ret_code: to_error_core(&err),
                err_msg: format!("{}", err),
                status: Status {is_registered: false},
            },
        }
    }
}

impl From<Result<()>> for ProviderServiceResult {
    fn from(result: Result<()>) -> Self {
        match result {
            Ok(_) => Self {
                ret_code: SUCCESS_CODE,
                err_msg: String::new(),
            },
            Err(err) => Self {
                ret_code: to_error_core(&err),
                err_msg: format!("{}", err),
            },
        }
    }
}
