mod response;
mod request;
pub use response::*;
pub use request::*;

pub trait IpcRequest {
    type IpcResponse: IpcResponse<IpcRequest = Self>;
    async fn try_into_p2p_response(&self) -> Result<IpcResponse, Error> {

    }
}

pub trait IpcResponse {
    type IpcRequest: IpcRequest<IpcResponse = Self>;
    async fn try_from_ipc_request(&self) -> Result<IpcRequest, Error>;

}