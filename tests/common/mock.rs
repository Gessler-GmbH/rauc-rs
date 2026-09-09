use futures_util::StreamExt;
use serde::Deserialize;

use zbus;
use zbus::connection::Builder;
use zbus::connection::socket::channel::Channel;
use zbus::{Connection, Guid, Message, MessageStream, Result};

/// Unique destination for the peer-to-peer mock, avoiding bus-name resolution.
pub const MOCK_DESTINATION: &str = ":1.42";

#[derive(Debug, Deserialize)]
struct Record {
    pub signature: String,
    pub body: Vec<u8>,
}

pub struct Dbus {
    client: Connection,
    server: Connection,
    messages: MessageStream,
}

impl Dbus {
    /// Creates two cross-wired in-memory D-Bus connections.
    ///
    /// `client` is passed to the production code.
    /// `server` acts as our fake D-Bus service.
    pub async fn new() -> Result<Self> {
        let guid = Guid::generate();
        let (sender, receiver) = Channel::pair();

        let server = Builder::authenticated_socket(sender, guid.clone())?
            .p2p()
            .build()
            .await?;

        // Subscribe before the client can send any requests.
        let messages = MessageStream::from(&server);

        let client = Builder::authenticated_socket(receiver, guid)?
            .p2p()
            .build()
            .await?;

        Ok(Self {
            client,
            server,
            messages,
        })
    }

    pub fn client(&self) -> Connection {
        self.client.clone()
    }

    /// Waits for one D-Bus method call and answers it using a recorded
    /// raw D-Bus body.
    pub async fn reply(&mut self, json: &str) -> Result<Message> {
        let record: Record = serde_json::from_str(json).expect("failed to parse D-Bus recording");

        let request = self
            .messages
            .next()
            .await
            .expect("D-Bus connection closed before receiving a request")?;

        let message = unsafe {
            Message::method_return(&request.header())?.build_raw_body(
                &record.body,
                record.signature.as_str(),
                Vec::new(),
            )?
        };

        self.server.send(&message).await?;

        Ok(request)
    }
}
