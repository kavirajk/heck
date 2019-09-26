use std::time;
use std::thread;

use reqwest;

use crate::config::Config;
use crate::storage::store_health_check;

pub struct Server<'s> {
    pub config: &'s Config,
    pub storage: redis::Client
}


impl<'a> Server<'a> {
    pub fn new(cfg: &'a Config, storage: redis::Client) -> Self {
        Server{
            config: cfg,
            storage: storage
        }
    }

    pub fn do_health_check(&self, endpoint: &str, timeout: time::Duration) -> Result<bool, reqwest::Error> {
        let client = match reqwest::Client::builder().timeout(timeout).build() {
            Ok(client) => client,
            Err(e) => return Err(e.into())
        };
    
        let resp = client.get(endpoint).send();
        match resp {
            Ok(resp) => Ok(resp.status() == reqwest::StatusCode::OK),
            Err(e) => Err(e)
        }
    }

    pub fn start(&self) {
        loop {
            let mut conn = self.storage.get_connection().expect("unable to get new connection");
            for origin in &self.config.servers {
                store_health_check(
                    &mut conn,
                    &origin.name,
                    self.do_health_check(&origin.endpoint, origin.timeout).unwrap(),
                    self.config.interval,
                ).expect(&format!("unable to store health info for {}", origin.name));
        }
            thread::sleep(self.config.interval);
        }
    }
}
