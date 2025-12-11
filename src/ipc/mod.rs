mod actor;
mod api;
mod api_trait;
mod context;
mod error;
mod protocol;


use std::{fmt::Display, net::SocketAddr, time::Duration};

use caretta_id::CarettaId;
use irpc::{Client, channel::oneshot, rpc::RemoteService, rpc_requests};
use n0_future::task::{self, AbortOnDropHandle};
use quinn::Endpoint;
use serde::{Deserialize, Serialize};

use crate::{error::CoreError, types::EndpointPublicKey};
pub use error::*;
pub use api::*;
pub use api_trait::*;
pub use actor::*;
pub use context::*;
pub use protocol::*;
pub type IpcResult<T> = std::result::Result<T, IpcError>;
pub type IpcActorResult<T> = std::result::Result<T, IpcActorError>;



