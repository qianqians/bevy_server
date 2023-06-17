use std::collections::HashMap;
use std::io::Write;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use tracing::{trace, debug, info, warn, error};

use thrift::protocol::{TCompactInputProtocolFactory, TCompactOutputProtocolFactory, TCompactInputProtocol, TCompactOutputProtocol, TInputProtocol, TOutputProtocol, TSerializable};
use thrift::transport::{TFramedReadTransportFactory, TFramedWriteTransportFactory, TFramedReadTransport, TFramedWriteTransport, ReadHalf, WriteHalf, TIoChannel, TTcpChannel, TBufferChannel, TReadTransport};
use thrift::server::{TServer, TProcessor};

use proto::dbproxy::{DbproxySyncHandler, DbproxySyncProcessor, DbEvent};
use proto::hub::HubDbproxyCallbackSyncClient;

use mongo::MongoProxy;
use queue::Queue;
use timer::utc_unix_timer;

mod db;

type HubInputProtocol = TCompactInputProtocol<TFramedReadTransport<ReadHalf<TTcpChannel>>>;
type HubOutputProtocol = TCompactOutputProtocol<TFramedWriteTransport<WriteHalf<TTcpChannel>>>;

pub struct DBProxyThriftServer {
    proxy: MongoProxy,
    queue: Queue<Box<db::DBEvent>>,
    hubs: Mutex<HashMap<String, *mut HubDbproxyCallbackSyncClient<HubInputProtocol, HubOutputProtocol>>>,
}

impl DBProxyThriftServer {
    pub fn new(_queue: Queue<Box<db::DBEvent>>, mongo_proxy:MongoProxy) -> DBProxyThriftServer {
        DBProxyThriftServer {
            proxy: mongo_proxy,
            queue: _queue,
            hubs: Mutex::new(HashMap::new()),
        }
    }

    fn cast_mut(&self) -> &mut Self {
        unsafe { &mut * (self as * const Self as * mut Self) }
    }

    fn deserialize(&self, data: Vec<u8>) -> Result<DbEvent, Box<dyn std::error::Error> > {
        let mut t = TBufferChannel::with_capacity(data.len(), 0);
        let _ = t.set_readable_bytes(&data);
        let mut i_prot = TCompactInputProtocol::new(t);
        let ev_data = DbEvent::read_from_in_protocol(&mut i_prot)?;
        Ok(ev_data)
    }

    async fn poll(&mut self) {
        let begin = utc_unix_timer();
        loop {
            let opt_ev_data = self.queue.deque();
            match opt_ev_data {
                None => break,
                Some(ev_data) => {
                    let mut mut_ev_data = ev_data;
                    mut_ev_data.do_event().await;
                }
            }
        }
        let tick = utc_unix_timer() - begin;

        if tick < 33 {
            thread::sleep(Duration::from_millis((33 - tick) as u64));
        }
    }
}