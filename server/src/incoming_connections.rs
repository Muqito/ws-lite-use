use crate::incoming_connection::IncomingConnection;
use crate::{TcpListener, WsError};
use futures_lite::StreamExt;

pub struct IncomingConnections<'a>(&'a mut TcpListener);
impl<'a> IncomingConnections<'a> {
    pub fn new(tcp_listener: &'a mut TcpListener) -> IncomingConnections<'a> {
        IncomingConnections(tcp_listener)
    }
    pub async fn next(&mut self) -> Option<Result<IncomingConnection, WsError>> {
        if let Some(Ok(connection)) = self.0.incoming().next().await {
            Some(Ok(IncomingConnection::new(connection)))
        } else {
            Some(Err(WsError::ConnectionError))
        }
    }
}
