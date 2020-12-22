use crate::incoming_connections::IncomingConnections;
use crate::TcpListener;

pub struct WsServer(TcpListener);
impl WsServer {
    pub async fn bind(input: &str) -> Result<WsServer, std::io::Error> {
        let tcp_listener = TcpListener::bind(input).await?;
        Ok(WsServer(tcp_listener))
    }
    pub fn connections<'a>(&'a mut self) -> IncomingConnections {
        IncomingConnections::new(&mut self.0)
    }
}
