use std::time::Duration;
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;
use p2p::{MessageHandlingStrategy, P2PError, P2PMessage, P2PMessageHandler, P2PResult};

#[tokio::main]
async fn main() -> P2PResult<()>{
    // This code is outside the task's actual scope - it just illustrates how the protocol handshake
    //  is intended to be used, especially the returned connection.

    SimpleLogger::new()
        .env()
        .with_colors(true)
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();

    let (mut connection, protocol_version) = p2p::handshake::open_with_handshake("localhost:12345", Duration::from_secs(2)).await?;

    info!("connect and handshake successful - negotiated protocol version {:?}", protocol_version);
    info!("starting regular message loop");
    connection.receive(&mut RegularMessageHandler {}).await?;

    Ok(())
}

struct RegularMessageHandler {

}
impl P2PMessageHandler for RegularMessageHandler {
    fn on_connection_closed(&mut self) {
        todo!()
    }

    fn on_message(&mut self, message: P2PMessage) -> MessageHandlingStrategy {
        todo!()
    }

    fn on_error(&mut self, err: P2PError) -> MessageHandlingStrategy {
        todo!()
    }
}