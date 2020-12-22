#![forbid(unsafe_code)]
#![feature(trait_alias)]
#![feature(array_methods)]
#[macro_use]
extern crate log;
use crate::api_error::APIError;
use async_channel::Sender;
use ws_lite_server::connection::TcpStreamNT;

pub mod api_error;
pub mod channel;
pub mod life_cycle;

#[derive(Debug)]
pub enum ServerMessage {
    Ping(channel::Sender<()>),
    NewClient((u32, TcpStreamNT, Sender<()>)),
}

pub async fn start_server() -> Result<(), APIError> {
    let (tx, rx) = channel::bounded::<ServerMessage>(1_000_000);
    let work = life_cycle::work::run(rx);
    let ws_incoming = life_cycle::ws_incoming::run(tx);

    let _ = futures_lite::future::try_zip(work, ws_incoming).await;

    Ok::<(), APIError>(())
}
