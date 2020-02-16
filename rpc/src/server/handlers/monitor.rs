// Copyright (c) SimpleStaking and Tezedge Contributors
// SPDX-License-Identifier: MIT

use std::convert::Infallible;

use warp::reply::json;

use crypto::hash::HashType;
use shell::shell_channel::BlockApplied;

use crate::encoding::base_types::*;
use crate::encoding::base_types::TimeStamp;
use crate::encoding::monitor::BootstrapInfo;
use crate::server::RpcServiceEnvironment;
use crate::ts_to_rfc3339;

pub fn bootstrapped(env: RpcServiceEnvironment) -> BootstrapInfo {
    let state_read = env.state().read().unwrap();

    match state_read.current_head().as_ref() {
        Some(current_head) => {
            let current_head: BlockApplied = current_head.clone();
            let block = HashType::BlockHash.bytes_to_string(&current_head.header().hash);
            let timestamp = ts_to_rfc3339(current_head.header().header.timestamp());
            BootstrapInfo::new(block.into(), TimeStamp::Rfc(timestamp))
        }
        None => BootstrapInfo::new(String::new().into(), TimeStamp::Integral(0))
    }
}

pub fn commit_hash() -> UniString {
    UniString::from(env!("GIT_HASH"))
}