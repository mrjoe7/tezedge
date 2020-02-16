// Copyright (c) SimpleStaking and Tezedge Contributors
// SPDX-License-Identifier: MIT

use std::net::SocketAddr;

use riker::actors::*;
use tokio::runtime::Handle;

use shell::shell_channel::{BlockApplied, ShellChannelMsg, ShellChannelRef, ShellChannelTopic};
use storage::persistent::PersistentStorage;
use tezos_api::client::TezosStorageInitInfo;

use crate::server::{RpcCollectedState, RpcCollectedStateRef, RpcServiceEnvironment, spawn_server};

pub type RpcServerRef = ActorRef<RpcServerMsg>;

/// Actor responsible for managing HTTP REST API and server, and to share parts of inner actor
/// system with the server.
#[actor(ShellChannelMsg)]
pub struct RpcServer {
    shell_channel: ShellChannelRef,
    state: RpcCollectedStateRef,
}

impl RpcServer {
    pub fn name() -> &'static str { "rpc-server" }

    fn new((shell_channel, state): (ShellChannelRef, RpcCollectedStateRef)) -> Self {
        Self { shell_channel, state }
    }

    pub fn actor(sys: &ActorSystem, shell_channel: ShellChannelRef, rpc_listen_address: SocketAddr, tokio_executor: &Handle, persistent_storage: &PersistentStorage, tezos_info: &TezosStorageInitInfo) -> Result<RpcServerRef, CreateError> {
        let shared_state = RpcCollectedStateRef::new(RpcCollectedState::new(load_current_head(persistent_storage), tezos_info.chain_id.clone()));
        let actor_ref = sys.actor_of(
            Props::new_args(Self::new, (shell_channel, shared_state.clone())),
            Self::name(),
        )?;

        // spawn RPC JSON server
        let env = RpcServiceEnvironment::new(sys.clone(), actor_ref.clone(), persistent_storage, &tezos_info.genesis_block_header_hash, shared_state, sys.log());
        tokio_executor.spawn(spawn_server(env));

        Ok(actor_ref)
    }
}

impl Actor for RpcServer {
    type Msg = RpcServerMsg;

    fn pre_start(&mut self, ctx: &Context<Self::Msg>) {
        self.shell_channel.tell(Subscribe {
            actor: Box::new(ctx.myself()),
            topic: ShellChannelTopic::ShellEvents.into(),
        }, ctx.myself().into());
    }

    fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Option<BasicActorRef>) {
        self.receive(ctx, msg, sender);
    }
}

impl Receive<ShellChannelMsg> for RpcServer {
    type Msg = RpcServerMsg;

    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: ShellChannelMsg, _sender: Sender) {
        match msg {
            ShellChannelMsg::BlockApplied(block) => {
                let current_head_ref = &mut *self.state.write().unwrap();
                match current_head_ref.current_head() {
                    Some(current_head) => {
                        let have_seen_newer_head = current_head.header().header.level() < block.header().header.level();
                        if have_seen_newer_head {
                            current_head_ref.set_current_head(Some(block));
                        }
                    }
                    None => {
                        current_head_ref.set_current_head(Some(block));
                    }
               };
            }
            _ => (/* Not yet implemented, do nothing */),
        }
    }
}

/// Load local head (block with highest level) from dedicated storage
fn load_current_head(persistent_storage: &PersistentStorage) -> Option<BlockApplied> {
    use storage::{BlockMetaStorage, BlockStorage, BlockStorageReader, IteratorMode, StorageError};

    BlockMetaStorage::new(persistent_storage)
        .iter(IteratorMode::End)
        .and_then(|meta_iterator|
            meta_iterator
                // unwrap a tuple of Result
                .filter_map(|(block_hash_res, meta_res)| block_hash_res.and_then(|block_hash| meta_res.map(|meta| (block_hash, meta))).ok())
                // we are interested in applied blocks only
                .filter(|(_, meta)| meta.is_applied())
                // get block with the highest level
                .max_by_key(|(_, meta)| meta.level())
                // get data for the block
                .map(|(block_hash, _)|
                    BlockStorage::new(persistent_storage)
                        .get_with_json_data(&block_hash)
                        .and_then(|data| data.map(|(block, json)| BlockApplied::new(block, json)).ok_or(StorageError::MissingKey))
                )
                .transpose()
        )
        .unwrap_or(None)
}