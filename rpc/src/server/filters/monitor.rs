// Copyright (c) SimpleStaking and Tezedge Contributors
// SPDX-License-Identifier: MIT

use warp::Filter;
use warp::reply::json;

use crate::server::filters::with_env;
use crate::server::handlers;
use crate::server::RpcServiceEnvironment;

/// Provide routing for `/monitor/` prefix.
pub fn routes(env: RpcServiceEnvironment) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("monitor"))
        .and(
            bootstrapped(env.clone())
                .or(commit_hash())
                .or(active_chains())
                .or(protocols())
                .or(valid_blocks())
                .or(heads())
        )
}

fn bootstrapped(env: RpcServiceEnvironment) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("bootstrapped")
        .and(with_env(env))
        .map(|env| json(&handlers::monitor::bootstrapped(env)))
}

fn commit_hash() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("commit_hash")
        .map(|| json(&handlers::monitor::commit_hash()))
}

fn active_chains() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("active_chains")
        .map(|| warp::reply::reply())
}

fn protocols() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("protocols")
        .map(|| warp::reply::reply())
}

fn valid_blocks() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("valid_blocks")
        .map(|| warp::reply::reply())
}

fn heads() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("heads" / String)
        .map(|_| warp::reply::reply())
}