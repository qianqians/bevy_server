use std::sync::{Mutex, Arc};
use std::marker::{Send, Sync};

use websocket::client::ClientBuilder;

use crate::wss_socket::{WSReader, WSWriter, WSSReader, WSSWriter};

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