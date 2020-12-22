use crate::channel::Sender;
use crate::{APIError, ServerMessage};
use async_std::task::JoinHandle;
use std::sync::Arc;
use ws_lite::message::Message;
use ws_lite_server::{WsError, WsServer};

pub fn run(sender: Sender<ServerMessage>) -> JoinHandle<Result<(), APIError>> {
    async_std::task::spawn(async move {
        let mut ws_server = WsServer::bind("[::1]:3333").await?;

        let mut connection: u32 = 0;
        let mut incoming = ws_server.connections();
        let sender = Arc::new(sender);
        while let Some(Ok(incoming_connection)) = incoming.next().await {
            connection += 1;
            let sender = Arc::clone(&sender);
            async_std::task::spawn(async move {
                if let Ok(mut ws_connection) = incoming_connection.accept().await {
                    /*                                        let mut close_connection = ws_connection.get_connection();
                    while let Some(Ok(dataframe)) = ws_connection.messages().next().await {
                        if dataframe.is_closed() {
                            break;
                        }
                        if let Some(text) = dataframe.text() {
                            let _ = ws_connection.send(Message::Text(text.to_string())).await;
                        }
                    }
                    close_connection.close();*/
                    let (tx, rx) = async_channel::bounded::<()>(1);
                    let mut close_connection = ws_connection.get_connection();
                    let _ = sender
                        .send(ServerMessage::NewClient((
                            connection,
                            ws_connection.get_connection(),
                            tx,
                        )))
                        .await;

                    let connection_lifecycle = async_std::task::spawn(async move {
                        while let Some(Ok(dataframe)) = ws_connection.messages().next().await {
                            if dataframe.is_closed() {
                                return Err(WsError::ConnectionClosed);
                            }
                            if let Some(text) = dataframe.text() {
                                let _ = ws_connection.send(Message::Text(text.to_string())).await;
                            }
                        }

                        Ok::<(), WsError>(())
                    });
                    let wait_for_close = async_std::task::spawn(async move {
                        let _ = rx.recv().await;
                        let _ = close_connection.close().await;
                        Err::<(), WsError>(WsError::ConnectionClosed)
                    });

                    if let Err(err) =
                        futures_lite::future::race(connection_lifecycle, wait_for_close).await
                    {
                        debug!("Connection: {}, {:#?}", connection, err);
                        // This is to close the connection faster than with the drop
                    }
                }
            });
        }

        Ok::<(), APIError>(())
    })
}
