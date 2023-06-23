use std::env;

use serde::{Deserialize, Serialize};

use tracing::{trace, debug, info, warn, error};

use proto::hub;
use config::{load_data_from_file, load_cfg_from_data};
use dbproxy::DBProxyServer;

#[derive(Deserialize, Serialize, Debug)]
struct DBProxyCfg {
    port: u16,
    mongo_url: String,
    log_level: String,
    log_file: String,
    log_dir: String
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg_file = &args[0];

    let cfg_data = match load_data_from_file(cfg_file.to_string()) {
        Err(e) => {
            error!("DBProxy load_data_from_file faild {}", cfg_file);
            return;
        },
        Ok(_cfg_data) => _cfg_data
    };
    let cfg = load_cfg_from_data::<DBProxyCfg>(&cfg_data);
}
