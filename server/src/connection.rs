use crate::messages::Messages;
use crate::{TcpStream, WsError};
use futures_lite::AsyncWriteExt;
use ws_lite::message::{Message, WriteMessage};

#[derive(Debug)]
pub struct TcpStreamNT {
    tcp_stream: TcpStream,
}
impl TcpStreamNT {
    pub async fn send<'a, I>(&mut self, data: I) -> Result<(), WsError>
    where
        //I: Into<&'a [u8]> + 'a,
        I: Into<WriteMessage>,
    {
        self.tcp_stream
            .write_all(I::into(data).as_ref())
            .await
            .map_err(|_| WsError::Unknown)
    }
    pub async fn close(&mut self) {
        let _ = self.send(Message::Close).await;
    }
}
pub struct WsConnection {
    pub(crate) tcp_stream: TcpStream,
}
impl WsConnection {
    pub fn new(tcp_stream: TcpStream) -> WsConnection {
        WsConnection { tcp_stream }
    }
    pub fn messages(&mut self) -> Messages {
        Messages::new(self)
    }
    pub fn get_connection(&self) -> TcpStreamNT {
        TcpStreamNT {
            tcp_stream: self.tcp_stream.clone(),
        }
    }

    pub async fn send<'a, I>(&mut self, data: I) -> Result<(), WsError>
    where
        //I: Into<&'a [u8]> + 'a,
        I: Into<WriteMessage>,
    {
        self.tcp_stream
            .write_all(I::into(data).as_ref())
            .await
            .map_err(|_| WsError::Unknown)
    }
}
