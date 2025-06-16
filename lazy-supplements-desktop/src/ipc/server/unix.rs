use std::{collections::VecDeque, path::Path, sync::Arc};

use lazy_supplements_core::error::Error;
use tokio::{io::Interest, net::UnixStream, sync::Mutex};

use crate::ipc::message::{RequestContent, Response, ResponseContent};

pub async fn listen<T>(path: T) -> Result<(), Error>
where T: AsRef<Path> {
    let stream = UnixStream::connect(path).await?;
    let write_que: Arc<Mutex<VecDeque<Vec<u8>>>> = Arc::new(Mutex::new(VecDeque::new()));
    let mut write_next: Option<Vec<u8>> = None;
    loop {
        let ready = stream.ready(Interest::READABLE).await?;
        if ready.is_readable() {
            let mut data = Vec::new();
            match stream.try_read(&mut data) {
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
            let write_que2 = write_que.clone();
            tokio::spawn( async move {
                let mut buf = Vec::new();
                let request: crate::ipc::message::Request = ciborium::from_reader_with_buffer(data.as_slice(), &mut buf).unwrap();
                let response_id = request.id;
                let response_content: ResponseContent = match request.content {
                    RequestContent::Ping => {
                        ResponseContent::Pong
                    }
                    RequestContent::ListPeers => todo!(),
                };
                let mut response_buf = Vec::new();
                if let Err(e) = ciborium::into_writer(&Response{
                    id: response_id,
                    content: response_content,
                }, &mut response_buf) {
                    todo!();
                };
                let mut que = write_que2.lock().await;
                que.push_back(response_buf);

            });
        } else if ready.is_writable()  {
            if let Some(x) = write_next.take() {

                match stream.try_write(&x) {
                    Ok(x) => {
                        println!("write {} bytes", x)
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(e) => {
                        return Err(e.into())
                    }
                }
            }
        }
        let mut locked_que = write_que.lock().await;
        write_next = locked_que.pop_front();
    }
}
