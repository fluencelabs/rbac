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

use fluence::{fce, CallParameters};
use fluence::WasmLoggerBuilder;

use crate::storage_api::{init, contains, add, delete};

mod storage_api;
mod errors;

pub const SUCCESS_CODE: i32 = 0;

pub(crate) type Result<T> = std::result::Result<T, errors::ProviderError>;

#[fce]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Status {
    pub is_registered: bool
}

pub fn main() {
    WasmLoggerBuilder::new()
        .with_log_level(log::Level::Info)
        .build()
        .unwrap();

    match init() {
        Ok(_) => log::info!("db created"),
        Err(e) => log::error!("sqlite db creation failed: {}", e),
    }
}

#[fce]
pub struct ProviderServiceResult {
    pub ret_code: i32,
    pub err_msg: String,
}

#[fce]
pub struct GetStatusServiceResult {
    pub ret_code: i32,
    pub err_msg: String,
    pub status: Status,
}

#[fce]
pub fn get_status() -> GetStatusServiceResult {
    let call_parameters: CallParameters = fluence::get_call_parameters();
    let peer_id = call_parameters.init_peer_id;

    let is_registered = contains(peer_id);

    return is_registered.into()
}

#[fce]
pub fn register(peer_id: String) -> ProviderServiceResult {
    let call_parameters: CallParameters = fluence::get_call_parameters();
    let init_peer_id = call_parameters.init_peer_id;
    let owner = call_parameters.service_creator_peer_id;

    if init_peer_id == owner || contains(init_peer_id).unwrap_or_else(|_| false) {
        add(peer_id).into()
    } else {
        return ProviderServiceResult {
            ret_code: SUCCESS_CODE,
            err_msg: String::new(),
        }
    }
}

#[fce]
pub fn remove(peer_id: String) -> ProviderServiceResult {
    let call_parameters: CallParameters = fluence::get_call_parameters();
    let init_peer_id = call_parameters.init_peer_id;
    let owner = call_parameters.service_creator_peer_id;

    if init_peer_id == owner || contains(init_peer_id).unwrap_or_else(|_| false) {
        return delete(peer_id).into();
    } else {
        return ProviderServiceResult {
            ret_code: SUCCESS_CODE,
            err_msg: String::new(),
        }
    }
}
