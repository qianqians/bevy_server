use std::sync::{Mutex, Arc};
use std::marker::{Send, Sync};

use websocket::client::ClientBuilder;

use crate::ws_socket::{WSReader, WSWriter};

pub struct WSConnect {
}

impl WSConnect {
    pub async fn connect<H: Send + Sync + 'static>(host:String) -> Result<(WSReader, WSWriter), Box<dyn std::error::Error>> {
        let _client = ClientBuilder::new(&host)
            .unwrap()
            .add_protocol("websocket")
            .connect_insecure()
            .unwrap();

        let ip = _client.peer_addr().unwrap();
        let (rd, wr) = _client.split().unwrap();
        let mut _wr_arc = Arc::new(Mutex::new(wr));
        let _wr_clone = _wr_arc.clone();

        Ok((
            WSReader::new(ip, rd, _wr_arc), 
            WSWriter::new(_wr_clone)
        ))
    }
}