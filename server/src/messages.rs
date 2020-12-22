use crate::connection::WsConnection;
use crate::WsError;
use futures_lite::AsyncReadExt;
use ws_lite::dataframe::DataFrame;

const MAX_MESSAGE_SIZE: usize = u16::MAX as usize + 10;
pub struct Messages<'a>(&'a mut WsConnection);
impl<'a> Messages<'a> {
    pub fn new(ws: &'a mut WsConnection) -> Messages<'a> {
        Messages(ws)
    }
}
impl<'a> Messages<'a> {
    pub async fn next(&mut self) -> Option<Result<DataFrame, WsError>> {
        let mut payload: [u8; MAX_MESSAGE_SIZE] = [0; MAX_MESSAGE_SIZE];
        match self.0.tcp_stream.read(&mut payload).await {
            // Connection was aborted
            Ok(size) if size == 0 => Some(Err(WsError::ConnectionClosed)),
            Ok(size) if size == MAX_MESSAGE_SIZE => Some(Err(WsError::InvalidPayload)),
            Ok(size) => Some(Ok(DataFrame::new(payload[0..size].to_vec()))),
            Err(_) => Some(Err(WsError::Unknown)),
        }
    }
}
