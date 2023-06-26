use std::sync::{Mutex, Arc};
use std::marker::{Send, Sync};

use websocket::client::ClientBuilder;

use crate::wss_socket::{WSSReader, WSSWriter};

pub struct WSSConnect {
}

impl WSSConnect {
    pub async fn connect<H: Send + Sync + 'static>(host:String) -> Result<(WSSReader, WSSWriter), Box<dyn std::error::Error>> {
        let _client = ClientBuilder::new(&host)
            .unwrap()
            .add_protocol("websocket")
            .connect_secure(None)
            .unwrap();

        let _s = Arc::new(Mutex::new(_client));
        let _s_clone = _s.clone();
        Ok((
            WSSReader::new(_s), 
            WSSWriter::new(_s_clone)
        ))
    }
}