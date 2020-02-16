// Copyright (c) SimpleStaking and Tezedge Contributors
// SPDX-License-Identifier: MIT

//! Filters are used to process incoming http request and then call corresponding handler

use serde::Serialize;
use warp::Filter;
use warp::http::StatusCode;
use warp::reply::Json;

use crate::server::RpcServiceEnvironment;

pub mod monitor;
pub mod block;

pub(crate) fn with_env(env: crate::server::RpcServiceEnvironment) -> impl Filter<Extract = (RpcServiceEnvironment, ), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || env.clone())
}

pub(crate) fn result_as_json<T>(res: &Result<T, failure::error::Error>) -> impl warp::Reply
    where
        T: Serialize,
{
    match res {
        Ok(t) => warp::reply::with_status(warp::reply::json(&t), StatusCode::OK),
        Err(err) => {
            warp::reply::with_status(warp::reply::json(&ErrorMessage {
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                message: err.to_string(),
            }), StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}