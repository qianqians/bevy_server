use std::env;

use serde::{Deserialize, Serialize};
use tracing::{info, error};
use uuid::Uuid;
use consulrs::api::check::common::AgentServiceCheckBuilder;
use consulrs::api::service::requests::RegisterServiceRequest;

use config::{load_data_from_file, load_cfg_from_data};
use dbproxy::DBProxyServer;
use health::HealthHandle;
use consul::ConsulImpl;
use local_ip::get_local_ip;
use log;

#[derive(Deserialize, Serialize, Debug)]
struct DBProxyCfg {
    consul_url: String,
    service_port: u16,
    health_port: u16,
    mongo_url: String,
    log_level: String,
    log_file: String,
    log_dir: String
}

#[tokio::main]
async fn main() {
    info!("dbproxy start!");

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

    let health_port = cfg.health_port;
    let health_host = format!("0.0.0.0:{}", health_port);
    let health_handle = HealthHandle::new(health_host).await;

    let host = format!("0.0.0.0:{}", cfg.service_port);
    let mut server = match DBProxyServer::new(cfg.mongo_url, host, health_handle).await {
        Err(e) => {
            error!("DBProxy DBProxyServer new faild {}!", e);
            return;
        },
        Ok(_s) => _s
    };

    let _name = format!("dbproxy_{}", Uuid::new_v4());
    let _local_ip = get_local_ip();
    let _health_host = format!("http://{_local_ip}:{health_port}/health");
    let mut consul_impl = ConsulImpl::new(cfg.consul_url);
    consul_impl.register(_name, Some(
        RegisterServiceRequest::builder()
            .address(_local_ip)
            .port(cfg.service_port)
            .check(AgentServiceCheckBuilder::default()
                .name("health_check")
                .interval("10s")
                .http(_health_host)
                .status("passing")
                .build()
                .unwrap()
            ),
        ),
    ).await;

    server.run().await;
    server.join().await;

    info!("dbproxy exit!");
}
