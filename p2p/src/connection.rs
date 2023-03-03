use log::debug;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::ToSocketAddrs;

use crate::{P2PError, P2PResult};

pub struct P2PConnection {
    stream: tokio::net::TcpStream,
    read_buf: [u8;32768],
    read_buf_pointer: usize,
}
impl P2PConnection {
    //TODO implement reconnect at a higher level

    pub async fn new(addr: impl ToSocketAddrs) -> P2PResult<P2PConnection> {
        let stream = tokio::net::TcpStream::connect(addr).await?;
        Ok(P2PConnection {
            stream,
            read_buf: [0;32768],
            read_buf_pointer: 0,
        })
    }

    pub async fn send(&mut self, payload: &[u8]) -> P2PResult<()> {
        Ok(self.stream.write_all(payload).await?)
    }

    pub async fn receive(&mut self, handler: &mut impl P2PMessageHandler) -> P2PResult<()>{
        loop {
            match self.stream.read(&mut self.read_buf[self.read_buf_pointer..]).await {
                Ok(0) => {
                    debug!("counterpart closed the connection with incomplete messages in our buffer");
                    return Ok(());
                }
                Ok(n) => {
                    self.read_buf_pointer += n;
                    while let Some(message) = self.message_from_buffer() {
                        match handler.on_message(message) {
                            MessageHandlingStrategy::ContinueReceiving(message) => {
                                if let Some(message) = message {
                                    self.send(&message.payload).await?;
                                }
                            }
                            MessageHandlingStrategy::StopReceiving(message) => {
                                if let Some(message) = message {
                                    self.send(&message.payload).await?;
                                }
                                //NB: unhandled messages remain in the buffer for a subsequent call to 'receive'
                                return Ok(());
                            }
                        }
                    }
                }
                Err(e) => {
                    match handler.on_error(e.into()) {
                        MessageHandlingStrategy::ContinueReceiving(message) => {
                            if let Some(message) = message {
                                self.send(&message.payload).await?;
                            }
                        }
                        MessageHandlingStrategy::StopReceiving(message) => {
                            if let Some(message) = message {
                                self.send(&message.payload).await?;
                            }
                            return Ok(());
                        }
                    }
                }
            }
        }
    }

    fn message_from_buffer(&mut self) -> Option<P2PMessage> {
        //TODO message parsing logic goes here
        if self.read_buf_pointer >= 100 {
            let result = P2PMessage { payload: self.read_buf[0..100].into() };
            self.read_buf_pointer -= 100;
            Some(result)
        }
        else {
            None
        }
    }
}

pub struct P2PMessage {
    //TODO protocol specific headers etc.
    pub payload: Vec<u8>
}

pub enum MessageHandlingStrategy {
    ContinueReceiving(Option<P2PMessage>),
    StopReceiving(Option<P2PMessage>),
}

pub trait P2PMessageHandler {
    fn on_connection_closed(&mut self);
    fn on_message(&mut self, message: P2PMessage) -> MessageHandlingStrategy;
    fn on_error(&mut self, err: P2PError) -> MessageHandlingStrategy;
}
