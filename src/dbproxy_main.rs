use std::env;

use serde::{Deserialize, Serialize};

use tracing::{error};

use config::{load_data_from_file, load_cfg_from_data};
use log;
use dbproxy::DBProxyServer;

#[derive(Deserialize, Serialize, Debug)]
struct DBProxyCfg {
    port: u16,
    mongo_url: String,
    log_level: String,
    log_file: String,
    log_dir: String
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg_file = &args[0];

    let cfg_data = match load_data_from_file(cfg_file.to_string()) {
        Err(e) => {
            error!("DBProxy load_data_from_file faild {}, {}!", cfg_file, e);
            return;
        },
        Ok(_cfg_data) => _cfg_data
    };
    let cfg = match load_cfg_from_data::<DBProxyCfg>(&cfg_data) {
        Err(e) => {
            error!("DBProxy load_cfg_from_data faild {}, {}!", cfg_data, e);
            return;
        },
        Ok(_cfg) => _cfg
    };
    log::init(cfg.log_level, cfg.log_dir, cfg.log_file);

    let host = format!("0.0.0.0:{}", cfg.port);
    let mut server = match DBProxyServer::new(cfg.mongo_url, host).await {
        Err(e) => {
            error!("DBProxy DBProxyServer new faild {}!", e);
            return;
        },
        Ok(_s) => _s
    };

    server.run().await;
    server.join().await;
}
