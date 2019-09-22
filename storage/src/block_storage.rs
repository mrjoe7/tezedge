// Copyright (c) SimpleStaking and Tezos-RS Contributors
// SPDX-License-Identifier: MIT

use std::sync::Arc;

use tezos_encoding::hash::BlockHash;

use crate::{BlockHeaderWithHash, StorageError};
use crate::persistent::{DatabaseWithSchema, Schema};
use crate::persistent::database::{IteratorWithSchema, IteratorMode};

pub type BlockStorageDatabase = dyn DatabaseWithSchema<BlockStorage> + Sync + Send;

/// Structure for representing in-memory db for - just for demo purposes.
#[derive(Clone)]
pub struct BlockStorage {
    db: Arc<BlockStorageDatabase>
}

impl BlockStorage {

    pub fn new(db: Arc<BlockStorageDatabase>) -> Self {
        BlockStorage { db }
    }

    pub fn insert(&mut self, block: &BlockHeaderWithHash) -> Result<(), StorageError> {
        self.db.put(&block.hash, block)
            .map_err(StorageError::from)
    }

    pub fn get(&self, block_hash: &BlockHash) -> Result<Option<BlockHeaderWithHash>, StorageError> {
        self.db.get(block_hash)
            .map_err(StorageError::from)
    }

    pub fn contains(&self, block_hash: &BlockHash) -> Result<bool, StorageError> {
        self.get(block_hash)
            .map_err(StorageError::from)
            .map(|v| v.is_some())
    }

    pub fn iter(&self, mode: IteratorMode<Self>) -> Result<IteratorWithSchema<Self>, StorageError> {
        self.db.iterator(mode)
            .map_err(StorageError::from)
    }
}

impl Schema for BlockStorage {
    const COLUMN_FAMILY_NAME: &'static str = "block_storage";
    type Key = BlockHash;
    type Value = BlockHeaderWithHash;
}