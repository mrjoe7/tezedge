// Copyright (c) SimpleStaking and Tezedge Contributors
// SPDX-License-Identifier: MIT

use std::ops::Deref;
use std::sync::{Arc, RwLock};

use getset::{Getters, Setters};
use riker::actors::ActorSystem;
use slog::Logger;

use crypto::hash::{BlockHash, ChainId, HashType};
use shell::shell_channel::BlockApplied;
use storage::persistent::PersistentStorage;

use crate::rpc_actor::RpcServerRef;

mod filters;
mod handlers;
mod service;

pub async fn spawn_server(env: RpcServiceEnvironment) {
    warp::serve(filters::monitor::routes(env)).run(([127, 0, 0, 1], 3039)).await;
}

/// Represents various collected information about
/// internal state of the node.
#[derive(Getters, Setters)]
pub struct RpcCollectedState {
    #[get = "pub(crate)"]
    #[set = "pub(crate)"]
    current_head: Option<BlockApplied>,
    #[get = "pub(crate)"]
    chain_id: ChainId,
}

impl RpcCollectedState {
    pub fn new(current_head: Option<BlockApplied>, chain_id: ChainId) -> Self {
        Self { current_head, chain_id }
    }
}

/// Thread safe reference to a shared RPC state
#[derive(Clone)]
pub struct RpcCollectedStateRef(Arc<RwLock<RpcCollectedState>>);

impl RpcCollectedStateRef {
    pub fn new(state: RpcCollectedState) -> Self {
        Self(Arc::new(RwLock::new(state)))
    }
}

impl Deref for RpcCollectedStateRef {
    type Target = Arc<RwLock<RpcCollectedState>>;

    fn deref(&self) ->  &Self::Target {
        &self.0
    }
}

/// Server environment parameters
#[derive(Getters, Clone)]
pub struct RpcServiceEnvironment {
    #[get = "pub(crate)"]
    sys: ActorSystem,
    #[get = "pub(crate)"]
    actor: RpcServerRef,
    #[get = "pub(crate)"]
    persistent_storage: PersistentStorage,
    #[get = "pub(crate)"]
    genesis_hash: String,
    #[get = "pub(crate)"]
    state: RpcCollectedStateRef,
    #[get = "pub(crate)"]
    log: Logger,
}

impl RpcServiceEnvironment {
    /// Create new instance of rpc environment
    pub fn new(sys: ActorSystem, actor: RpcServerRef, persistent_storage: &PersistentStorage, genesis_hash: &BlockHash, state: RpcCollectedStateRef, log: Logger) -> Self {
        Self {
            sys,
            actor,
            persistent_storage: persistent_storage.clone(),
            genesis_hash: HashType::BlockHash.bytes_to_string(genesis_hash),
            state,
            log
        }
    }
}