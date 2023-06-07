use std::time::{SystemTime, UNIX_EPOCH};

use crate::{account::Account, StorageError};

use super::schema::{accounts, messages};
use diesel::prelude::*;
use serde_json::json;

/// Placeholder type for messages returned from the Store.
#[derive(Queryable, Debug)]
pub struct DecryptedMessage {
    pub id: i32,
    pub created_at: i64,
    pub convo_id: String,
    pub addr_from: String,
    pub content: Vec<u8>,
}

/// Placeholder type for messages being inserted into the store. This type is the same as
/// DecryptedMessage expect it does not have an `id` feild. The field is generated by the
/// store when it is inserted.
#[derive(Insertable, Clone, PartialEq, Debug)]
#[diesel(table_name = messages)]
pub struct NewDecryptedMessage {
    pub created_at: i64,
    pub convo_id: String,
    pub addr_from: String,
    pub content: Vec<u8>,
}

impl NewDecryptedMessage {
    pub fn new(convo_id: String, addr_from: String, content: Vec<u8>) -> Self {
        Self {
            created_at: now(),
            convo_id,
            addr_from,
            content,
        }
    }
}
impl PartialEq<DecryptedMessage> for NewDecryptedMessage {
    fn eq(&self, other: &DecryptedMessage) -> bool {
        self.created_at == other.created_at
            && self.convo_id == other.convo_id
            && self.addr_from == other.addr_from
            && self.content == other.content
    }
}

fn now() -> i64 {
    let start = SystemTime::now();
    start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos() as i64
}

#[derive(Queryable, Debug)]
pub struct StoredAccount {
    pub id: i32,
    pub created_at: i64,
    pub serialized_key: Vec<u8>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = accounts)]
pub struct NewStoredAccount {
    pub created_at: i64,
    pub serialized_key: Vec<u8>,
}
impl TryFrom<&Account> for NewStoredAccount {
    type Error = StorageError;
    fn try_from(account: &Account) -> Result<Self, StorageError> {
        Ok(Self {
            created_at: now(),
            serialized_key: serde_json::to_vec(account).map_err(|e| {
                StorageError::Store(format!(
                    "could not initialize model:NewStoredAccount -- {}",
                    e.to_string()
                ))
            })?,
        })
    }
}
