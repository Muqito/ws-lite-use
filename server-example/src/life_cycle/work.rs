use crate::channel::Receiver;
use crate::{APIError, ServerMessage};
use async_std::task::JoinHandle;
use ws_lite::message::Message;

pub fn run(recv: Receiver<ServerMessage>) -> JoinHandle<Result<(), APIError>> {
    async_std::task::spawn(async move {
        while let Ok(message) = recv.recv().await {
            async_std::task::spawn(async move {
                match message {
                    ServerMessage::Ping(tx) => {
                        let _ = tx.send(()).await;
                    }
                    ServerMessage::NewClient((1, mut tcp, _)) => loop {
                        let _ = tcp
                            .send(Message::Text(format!("Hello {} from server", 1)))
                            .await;
                        std::thread::sleep(std::time::Duration::from_secs(2));
                    },
                    ServerMessage::NewClient((id, mut tcp, tx_close)) => {
                        let _ = tcp
                            .send(Message::Text(format!(
                                "Hello {} from server, closing connection in 5 sec",
                                id,
                            )))
                            .await;
                        std::thread::sleep(std::time::Duration::from_secs(5));
                        let _ = tx_close.send(()).await;
                    }
                }
            });
        }
        Ok::<(), APIError>(())
    })
}
