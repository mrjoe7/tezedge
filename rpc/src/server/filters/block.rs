// Copyright (c) SimpleStaking and Tezedge Contributors
// SPDX-License-Identifier: MIT

use warp::Filter;
use warp::reply::json;

use crate::server::filters::{result_as_json, with_env};
use crate::server::handlers;
use crate::server::RpcServiceEnvironment;

/// Provide routing for `/chains/:chain_id/blocks/` prefix.
pub fn routes(env: RpcServiceEnvironment) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(chains_block_id(env.clone()))
}

fn chains_block_id(env: RpcServiceEnvironment) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("chains" /  String / "blocks" / String)
        .and(with_env(env))
        .map(|chain_id, block_id, env| result_as_json(&handlers::block::chains_block_id(chain_id, block_id, env)))
}

