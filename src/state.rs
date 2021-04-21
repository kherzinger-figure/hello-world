use cosmwasm_std::Storage;
use cosmwasm_storage::{bucket, bucket_read, Bucket, ReadonlyBucket};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub static NAMESPACE_MESSAGE: &[u8] = b"message";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MessageState {
    pub id: String,
    pub message: String,
}

pub fn get_message_storage(storage: &mut dyn Storage) -> Bucket<MessageState> {
    bucket(storage, NAMESPACE_MESSAGE)
}

pub fn get_message_storage_read(storage: &dyn Storage) -> ReadonlyBucket<MessageState> {
    bucket_read(storage, NAMESPACE_MESSAGE)
}
