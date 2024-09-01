use url::Url;

use crate::proxy_manager::ProxyManager;

#[derive(Debug, Clone)]
pub struct ProxyFactory {
    manager: ProxyManager,
}

impl ProxyFactory {
    pub fn new() -> Self {
        Self {
            manager: ProxyManager::new(),
        }
    }

    pub fn get_proxies(&self, url: Url) -> Vec<Url> {
        self.manager.get_proxies(url)
    }
}
