use std::path::Path;
use tokio::{io::Interest, net::UnixStream};

use crate::{
    error::Error,
    ipc::message::{Request, Response, ResponseContent},
};


pub async fn request<T, U>(path: T, request: U) -> Result<ResponseContent, Error>
where
    T: AsRef<Path>,
    U: Into<Request>
{
    let stream = UnixStream::connect(path).await?;
    let ready = stream.ready(Interest::WRITABLE).await?;
    let request: Request = request.into();
    let mut response_buf = Vec::new();
    if let Err(e) = ciborium::into_writer(&request, &mut response_buf) {
        todo!();
    };
    match stream.try_write(&response_buf) {
        Ok(x) => {
            println!("write {} bytes", x)
        }
        Err(e) => {
            return Err(e.into())
        }
    }
    loop {
        let ready_write = stream.ready(Interest::READABLE).await?;
        let mut read_buf : Vec<u8> = Vec::new();
        match stream.try_read_buf(&mut read_buf) {
            Ok(x) => {
                println!("read {} bytes", x)
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into())
            }
        }
        let mut buf : Vec<u8> = Vec::new();
        let response: Response = ciborium::from_reader_with_buffer(read_buf.as_slice(), &mut buf)?;
        if response.id == request.id {
            return Ok(response.content)
        }
    }
}