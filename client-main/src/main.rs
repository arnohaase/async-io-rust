use std::time::Duration;
use log::LevelFilter;
use simple_logger::SimpleLogger;
use p2p::P2PResult;

#[tokio::main]
async fn main() -> P2PResult<()>{
    SimpleLogger::new()
        .env()
        .with_colors(true)
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();

    let (connection, protocol_version) = p2p::handshake::open_with_handshake("localhost:12345", Duration::from_secs(2)).await?;

    println!("connect and handshake successful - negotiated protocol version {:?}", protocol_version);

    Ok(())
}
