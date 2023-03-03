use std::time::Duration;
use tokio::net::ToSocketAddrs;
use crate::{MessageHandlingStrategy, P2PConnection, P2PError, P2PMessage, P2PMessageHandler, P2PProtocolVersion, P2PResult};


pub async fn open_with_handshake(addr: impl ToSocketAddrs, timeout: Duration) -> P2PResult<(P2PConnection, P2PProtocolVersion)> {
    let future = open_with_handshake_internal(addr);
    let result = tokio::time::timeout(timeout, future).await?;
    result
}
async fn open_with_handshake_internal(addr: impl ToSocketAddrs) -> P2PResult<(P2PConnection, P2PProtocolVersion)> {
    let mut connection = P2PConnection::new(addr).await?;

    connection.send(&P2PHandshake::my_version_message()).await?;

    let mut handshake = P2PHandshake {
        other_version: None,
        my_version_acknowledged: false,
        error: None,
    };
    connection.receive(&mut handshake).await?;

    if let Some(error) = handshake.error {
        return Err(error);
    }

    if handshake.my_version_acknowledged {
        if let Some(other_protocol_version) = &handshake.other_version {
            // TODO more logic for extracting the shared protocol version, features etc.
            let protocol_version = other_protocol_version.clone();

            return Ok((connection, protocol_version));
        }
    }

    Err("failed handshake".into()) // connection is closed when it goes out of scope
}


struct P2PHandshake {
    other_version: Option<P2PProtocolVersion>,
    my_version_acknowledged: bool,
    error: Option<P2PError>,
}
impl P2PHandshake {
    fn my_version_message() -> Vec<u8> {
        b"todo message serialization goes here".to_vec()
    }
}

impl P2PMessageHandler for P2PHandshake {
    fn on_connection_closed(&mut self) {
        // nothing to be done
    }

    fn on_message(&mut self, message: P2PMessage) -> MessageHandlingStrategy {
        let mut response = None;

        //TODO real messages, real comparisons
        if &message.payload == &b"other V1" {
            self.other_version = Some(P2PProtocolVersion::V1);
            response = Some(P2PMessage { payload: b"version_ack".to_vec() });
        }
        if &message.payload == &b"other V2" {
            self.other_version = Some(P2PProtocolVersion::V2);
            response = Some(P2PMessage { payload: b"version_ack".to_vec() });
        }
        if &message.payload == &b"version_ack" { self.my_version_acknowledged = true; }

        if self.my_version_acknowledged && self.other_version.is_some() {
            MessageHandlingStrategy::StopReceiving(response)
        }
        else {
            MessageHandlingStrategy::ContinueReceiving(response)
        }
    }

    fn on_error(&mut self, err: P2PError) -> MessageHandlingStrategy {
        self.error = Some(err);
        MessageHandlingStrategy::StopReceiving(None)
    }
}
