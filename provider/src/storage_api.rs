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

use crate::Result;

use fce_sqlite_connector::Connection;
use fce_sqlite_connector::Value::String as VString;

use once_cell::sync::Lazy;

static SQLITE: Lazy<Connection> = Lazy::new(|| Connection::open(":memory:").unwrap());

pub fn init() -> Result<()> {
    let init_sql = "CREATE TABLE IF NOT EXISTS providers_list(\
        peer_id TEXT PRIMARY KEY
    );";

    SQLITE.execute(init_sql).map_err(Into::into)
}

pub fn add(peer_id: String) -> Result<()> {
    let add_peer_sql = "INSERT INTO providers_list (peer_id) VALUES (?)";
    let mut cursor = SQLITE.prepare(add_peer_sql)?.cursor();
    cursor.bind(&[VString(peer_id)])?;
    cursor.next()?;

    Ok(())
}

pub fn contains(peer_id: String) -> Result<bool> {
    let sql = "SELECT * FROM providers_list WHERE peer_id = ?";

    let mut cursor = SQLITE.prepare(sql)?.cursor();
    cursor.bind(&[VString(peer_id)])?;

    let result = cursor
        .next()?
        .map(|_| true)
        .unwrap_or_else(|| false);

    Ok(result)
}

pub fn delete(peer_id: String) -> Result<()> {
    let sql = "DELETE * FROM providers_list WHERE peer_id = ?";

    let mut cursor = SQLITE.prepare(sql)?.cursor();
    cursor.bind(&[VString(peer_id)])?;

    cursor.next()?;

    Ok(())
}
