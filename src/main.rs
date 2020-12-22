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


#![feature(once_cell)]

use fluence::fce;
use fluence::WasmLoggerBuilder;
use std::lazy::OnceCell;
use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::sync::Mutex;


#[fce]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Status {
    pub is_registered: bool
}

fn global_data() -> &'static Mutex<HashMap<String, Status>> {
    static INSTANCE: OnceCell<Mutex<HashMap<String, Status>>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let mut m = HashMap::new();
        Mutex::new(m)
    })
}

pub fn main() {
    WasmLoggerBuilder::new()
        .with_log_level(log::Level::Info)
        .build()
        .unwrap();
}

#[fce]
pub fn get_status(peer_id: String) -> Status {
    let data = global_data().lock().unwrap();

    match data.get(peer_id.as_str()) {
        None => {
            Status {
                is_registered: true
            }
        }
        Some(status) => status.clone()
    }
}

#[fce]
pub fn register(peer_id: String) {
    let mut data = global_data().lock().unwrap();

    data.insert(peer_id, Status {is_registered: true});
}

#[fce]
pub fn remove(peer_id: String) {
    let mut data = global_data().lock().unwrap();

    data.remove(peer_id.as_str());
}


