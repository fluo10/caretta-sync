mod node;
use serde::{de::DeserializeOwned, Serialize};
use uuid::Uuid;

use crate::{async_convert::{AsyncTryFrom, AsyncTryInto}, error::Error};

pub trait Message: DeserializeOwned + Sized + Serialize  {
    fn into_writer<W: std::io::Write>(&self, writer: W) -> Result<(), ciborium::ser::Error<std::io::Error>> {
        ciborium::into_writer(self, writer)
    }
    fn into_vec_u8(&self) -> Result<Vec<u8>, ciborium::ser::Error<std::io::Error>> {
        let mut buf: Vec<u8> = Vec::new();
        self.into_writer(&mut buf)?;
        Ok(buf)
    }
    fn from_reader<R: std::io::Read>(reader: R) -> Result<Self, ciborium::de::Error<std::io::Error>> {
        ciborium::from_reader(reader)
    }
}

pub trait Request<T>: Into<T> + From<T> + AsyncTryInto<Self::Response>
where T: Message {
    type Response: Response<T, Request = Self>;
    async fn send_p2p(self) -> Result<Self::Response, Error>;
    }

pub trait Response<T>: Into<T> + From<T> + AsyncTryFrom<Self::Request>
where T: Message{
    type Request: Request<T, Response = Self>;
    async fn from_request_with_local(req: Self::Request) -> Result<Self,Error>;
    async fn from_request_with_p2p(req: Self::Request) -> Result<Self, Error> {
        todo!()
    }
}

pub trait FromDatabase {
    async fn from_storage();
}


pub trait P2pRequest<T>: Into<T> + From<T> 
where T: Message {
    type P2pResponse: P2pResponse<T, P2pRequest = Self>;
    async fn send_p2p(&self) -> Result<Self::P2pResponse, crate::p2p::error::P2pError>{
        todo!()
    }
}
pub trait P2pResponse<T>: Into<T> + From<T> + AsyncTryFrom<(Self::P2pRequest)>
where T: Message {
    type P2pRequest: P2pRequest<T, P2pResponse = Self>;
    async fn try_from_p2p_request(source: Self::P2pRequest) -> Result<Self, crate::p2p::error::P2pError>;
}