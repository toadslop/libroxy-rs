use std::cmp::Ordering;

use crate::{network_monitor::NetworkMonitor, plugins::Plugin};
use log::debug;
use url::Url;

#[derive(Debug, Clone)]
pub struct ProxyManager {
    // parent_instance: TODO
    config_plugins: Vec<Plugin>,
    // packrunner_plugins: TODO
    network_monitor: NetworkMonitor,
    #[cfg(feature = "reqwest")]
    client: reqwest::Client,
    // config_plugin:  TODO
    // config_option: TODO
    force_online: bool,
    online: bool,
    wpad: bool,
    pac_data: Option<Vec<u8>>,
    pac_url: Option<Url>,
}

impl ProxyManager {
    pub fn new() -> Self {
        todo!()
    }

    pub fn get_proxies(&self, url: Url) -> Vec<Url> {
        todo!()
    }

    fn on_network_changed(&mut self, monitor: NetworkMonitor, network_available: bool) {
        debug!("Network connection changed, clearing pac data");
        self.wpad = false;
        self.online = network_available;
        self.pac_data = None;
        self.pac_url = None;
    }

    fn config_order_compare(a: &Plugin, b: &Plugin) -> Ordering {
        a.cmp(b)
    }
}

// TODO: implement sorting for configs
