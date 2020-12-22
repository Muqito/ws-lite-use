use crate::connection::WsConnection;
use crate::{TcpStream, WsError};
use futures_lite::{AsyncReadExt, AsyncWriteExt};
use ws_lite::accept_connection::AcceptResponse;

pub struct IncomingConnection {
    pub(crate) tcp_stream: TcpStream,
}
impl IncomingConnection {
    pub fn new(tcp_stream: TcpStream) -> IncomingConnection {
        IncomingConnection { tcp_stream }
    }
    pub async fn accept(self) -> Result<WsConnection, WsError> {
        let mut tcp_stream = self.tcp_stream;
        let _ = tcp_stream.set_nodelay(true);

        let mut buffer = [0; 1024];
        if tcp_stream.read(&mut buffer).await.is_err() {
            return Err(WsError::ConnectionError);
        }

        if let Some(accept_response) = AcceptResponse::from_buffer(&buffer) {
            if tcp_stream.write_all(accept_response.as_ref()).await.is_ok() {
                Ok(WsConnection::new(tcp_stream))
            } else {
                Err(WsError::ConnectionError)
            }
        } else {
            Err(WsError::ConnectionError)
        }
    }
}
