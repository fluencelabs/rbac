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

use std::collections::HashMap;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use fluence::CallParameters;
use std::ops::Deref;

static INSTANCE: OnceCell<Mutex<Option<Tetraplet>>> = OnceCell::new();

#[fce]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Tetraplet {
    pub peer_pk: String,
    pub service_id: String,
    pub fn_name: String,
    pub json_path: String,
}

fn global_data() -> &'static Mutex<Option<Tetraplet>> {
    INSTANCE.get_or_init(|| {
        <_>::default()
    })
}

pub fn main() {
    WasmLoggerBuilder::new()
        .with_log_level(log::Level::Info)
        .build()
        .unwrap();
}

#[fce]
pub fn set_tetraplet(peer_id: String, service_id: String, fn_name: String, path: String) {
    let mut data = global_data().lock();

    let tetraplet = Tetraplet {
        peer_pk: peer_id,
        service_id,
        fn_name,
        json_path: path,
    };

    data.replace(tetraplet);
}

#[fce]
pub fn is_authorized(auth: bool) -> bool {
    let data = global_data().lock();

    match data.deref() {
        None => false,
        Some(t) => {
            let call_parameters: CallParameters = fluence::get_call_parameters();
            let st = &call_parameters.tetraplets[0][0];

            return st.peer_pk == t.peer_pk && st.function_name == t.fn_name
                && st.service_id == t.service_id &&
                st.json_path == t.json_path;
        }
    }
}

