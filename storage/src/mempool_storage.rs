// Copyright (c) SimpleStaking and Tezedge Contributors
// SPDX-License-Identifier: MIT

use std::sync::Arc;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crypto::hash::{HashType, OperationHash};
use tezos_messages::p2p::binary_message::MessageHash;
use tezos_messages::p2p::encoding::operation::{MempoolOperationType, OperationMessage};

use crate::{num_from_slice, StorageError};
use crate::persistent::{BincodeEncoded, Decoder, Encoder, KeyValueSchema, KeyValueStoreWithSchema, PersistentStorage, SchemaError};

/// Convenience type for operation meta storage database
pub type MempoolStorageKV = dyn KeyValueStoreWithSchema<MempoolStorage> + Sync + Send;

/// Operation metadata storage
#[derive(Clone)]
pub struct MempoolStorage {
    kv: Arc<MempoolStorageKV>
}

impl MempoolStorage {
    pub fn new(persistent_storage: &PersistentStorage) -> Self {
        Self { kv: persistent_storage.kv() }
    }

    #[inline]
    pub fn put_pending(&mut self, message: OperationMessage, time_to_live: SystemTime) -> Result<(), StorageError> {
        self.put(MempoolOperationType::Pending, message, time_to_live)
    }

    #[inline]
    pub fn put_known_valid(&mut self, message: OperationMessage, time_to_live: SystemTime) -> Result<(), StorageError> {
        self.put(MempoolOperationType::KnownValid, message, time_to_live)
    }

    #[inline]
    pub fn put(&mut self, operation_type: MempoolOperationType, operation: OperationMessage, time_to_live: SystemTime) -> Result<(), StorageError> {
        let key = MempoolKey {
            operation_type,
            operation_hash: operation.message_hash()?
        };
        let value = MempoolValue {
            operation,
            time_to_live
        };

        self.kv.put(&key, &value)
            .map_err(StorageError::from)
    }

    #[inline]
    pub fn get(&self, operation_type: MempoolOperationType, operation_hash: OperationHash) -> Result<Option<OperationMessage>, StorageError> {
        let key = MempoolKey {operation_type, operation_hash };
        self.kv.get(&key)
            .map(|value| value.map(|value| value.operation))
            .map_err(StorageError::from)
    }
}

impl KeyValueSchema for MempoolStorage {
    type Key = MempoolKey;
    type Value = MempoolValue;

    #[inline]
    fn name() -> &'static str {
        "mempool_storage"
    }
}



#[derive(Serialize, Deserialize, Debug)]
pub struct MempoolKey {
    operation_type: MempoolOperationType,
    operation_hash: OperationHash
}

impl MempoolKey {
    const LEN_TYPE: usize = 1;
    const LEN_HASH: usize = HashType::OperationHash.size();
    const LEN_KEY: usize = Self::LEN_TYPE + Self::LEN_HASH;

    const IDX_TYPE: usize = 0;
    const IDX_HASH: usize = Self::IDX_TYPE + Self::LEN_TYPE;
}

impl Encoder for MempoolKey {
    fn encode(&self) -> Result<Vec<u8>, SchemaError> {
        if self.operation_hash.len() == Self::LEN_HASH {
            let mut bytes = Vec::with_capacity(Self::LEN_KEY);
            bytes.push(self.operation_type.to_u8());
            bytes.extend(&self.operation_hash);
            Ok(bytes)
        } else {
            Err(SchemaError::EncodeError)
        }
    }
}

impl Decoder for MempoolKey {
    fn decode(bytes: &[u8]) -> Result<Self, SchemaError> {
        if bytes.len() == Self::LEN_KEY {
            let operation_type = MempoolOperationType::from_u8(num_from_slice!(bytes, Self::IDX_TYPE, u8))
                .map_err(|_| SchemaError::DecodeError)?;
            let operation_hash = bytes[Self::IDX_HASH..Self::IDX_HASH + Self::LEN_HASH].to_vec();
            Ok(MempoolKey { operation_type, operation_hash })
        } else {
            Err(SchemaError::DecodeError)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MempoolValue {
    operation: OperationMessage,
    time_to_live: SystemTime,
}

impl BincodeEncoded for MempoolValue { }