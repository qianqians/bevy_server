use tracing::{info, error};

use consulrs::client::{ConsulClient, ConsulClientSettingsBuilder};
use consulrs::api::service::requests::RegisterServiceRequestBuilder;
use consulrs::service;

pub struct ConsulImpl {
    client: ConsulClient
}

impl ConsulImpl  {
    pub fn new(consul_url: String) -> ConsulImpl {
        let setting = ConsulClientSettingsBuilder::default()
            .address(consul_url)
            .build()
            .unwrap();
        let _client = ConsulClient::new(setting).unwrap();
        ConsulImpl { client: _client }
    }

    pub async fn register(&mut self, name: String, opts: Option<&mut RegisterServiceRequestBuilder>) {
        match service::register(&self.client, &name, opts).await {
            Err(e) => {
                error!("consul register err:{}!", e);
            },
            Ok(_) => {
                info!("consul register success!");
            }
        }
    }
}