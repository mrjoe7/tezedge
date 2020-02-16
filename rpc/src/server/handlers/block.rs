// Copyright (c) SimpleStaking and Tezedge Contributors
// SPDX-License-Identifier: MIT

use slog::Logger;

use storage::persistent::PersistentStorage;

use crate::encoding::chain::BlockInfo;
use crate::helpers::FullBlockInfo;
use crate::server::{RpcCollectedStateRef, RpcServiceEnvironment};
use crate::server::service::fns;

pub fn chains_block_id(chain_id: String, block_id: String, env: RpcServiceEnvironment) -> Result<Option<BlockInfo>, failure::Error> {
    if chain_id == "main" {
        if block_id == "head" {
            Ok(fns::get_full_current_head(env.state()).map(BlockInfo::from))
        } else {
            fns::get_full_block(&block_id, env.persistent_storage(), env.state()).map(|res| res.map(BlockInfo::from))
        }
    } else {
        Ok(None)
    }
}