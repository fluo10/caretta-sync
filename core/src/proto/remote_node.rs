use std::{pin::Pin, time::Duration};


use futures::{future::Remote, Stream};
use iroh::{endpoint::{DirectAddrInfo, RemoteInfo}, PublicKey};
use tonic::{Request, Response, Status, Streaming};
use tripod_id::Double;

use crate::{data::local::{LocalRecordId, RemoteNodeRecord}, global::IROH_ENDPOINT, error::Error, proto::{error::{ProtoDeserializeError, ProtoSerializeError}, generated::remote_node::*}};


impl TryFrom<(iroh::endpoint::Source, Duration)> for RemoteNodeSource {
    type Error = ProtoSerializeError;
    fn try_from(src: (iroh::endpoint::Source, Duration)) -> Result<Self, Self::Error> {
        let (source, duration )= src;
        Ok(Self {
            source: source.to_string(),
            duration: Some(duration.try_into()?),
        })
    }
}

impl TryFrom<RemoteNodeIdentifier> for RemoteNodeRecord<LocalRecordId> {
    type Error = Error;
    fn try_from(value: RemoteNodeIdentifier) -> Result<Self, Self::Error> {
        Ok(match value.identifier.ok_or(ProtoDeserializeError::MissingField("RemoteNodeIdentifier.identifier"))? {
            remote_node_identifier::Identifier::PublicKey(x) => Self::get_or_insert_by_public_key(&x.try_into()?)?,
            remote_node_identifier::Identifier::Id(x) => Self::get_by_public_id(&x.try_into()?)?
        })
    }

}


impl TryFrom<(tripod_id::Double, RemoteInfo)> for RemoteNodeInfo {
    type Error = ProtoSerializeError;
    fn try_from(value: (tripod_id::Double, RemoteInfo)) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Some(value.0.into()),
            public_key: Some(value.1.node_id.into()),
            relay_url: value.1.relay_url.map_or(String::from(""), |x| x.relay_url.to_string()),
            addrs: value.1.addrs.into_iter()
                .map(|x| DirectAddrInfoMessage::try_from(x))
                .collect::<Result<Vec<DirectAddrInfoMessage>,Self::Error>>()?,
            conn_type: value.1.conn_type.to_string(),
            latency: value.1.latency.map(|x| x.try_into()).transpose()?,
            last_used: value.1.last_used.map(|x| x.try_into()).transpose()?,
        })
    }
}

impl TryFrom<DirectAddrInfo> for RemoteNodeDirectAddrInfo {
    type Error = ProtoSerializeError;
    fn try_from(value: DirectAddrInfo) -> Result<Self, Self::Error> {
        Ok(RemoteNodeDirectAddrInfo {
            addr: value.addr.to_string(),
            latency: value.latency.map(|x| x.try_into()).transpose()?,
            last_control: value.last_control.map(|x| LastControlMessage::try_from(x)).transpose()?,
            last_payload: value.last_payload.map(|x| x.try_into()).transpose()?,
            last_alive: value.last_alive.map(|x| x.try_into()).transpose()?,
            sources: value.sources.into_iter().map(|x| SourceMessage::try_from(x)).collect::<Result<Vec<SourceMessage>, Self::Error>>()?
        })
    }
}

pub struct RemoteNodeServer{}

#[tonic::async_trait]
impl remote_node_server::RemoteNode for RemoteNodeServer {
    type ListStream = Pin<Box<dyn Stream<Item = Result<RemoteNodeListResponse, Status>> + Send>>;
    async fn info(&self, request: Request<RemoteNodeInfoRequest>) -> Result<Response<RemoteNodeInfoResponse>, Status> {
        let request = request.into_inner();
        let remote_node: RemoteNodeRecord<LocalRecordId> = request.remote_node.ok_or(Status::invalid_argument("remote_node is required."))?.try_into()?;
        let remote_info: RemoteNodeInfo = IROH_ENDPOINT.get_unchecked().remote_info(remote_node.public_key).ok_or(Status::not_found(format!("node {:?} is not found", remote_node)))?.into();
        Ok(Response::new(RemoteNodeInfoResponse{
            remote_node_info: Some(remote_info)
        }))
    } 
    async fn list(&self, request: Request<Streaming<RemoteNodeListRequest>>) 
        -> Result<Response<Self::ListStream>, Status> {
        let iter = IROH_ENDPOINT.get_unchecked().remote_info_iter()
            .map(|x| {
                RemoteNodeInfo::try_from(x).map(|x| RemoteNodeListResponse {
                    remote_node: x.into()
                }).or_else(|e| {
                    Err(Status::from_error(Box::new(e)))
                })
            });
        let stream = futures::stream::iter(iter);
        Ok(Response::new(Box::pin(stream)))
    }
}