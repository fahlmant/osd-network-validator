extern crate clap;
extern crate serde_yaml;

use clap::Parser;
use std::fs;
use std::time::Duration;
use std::thread;
use std::collections::HashMap;

#[macro_use]
extern crate serde_derive;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
// Represents an endpoint configuration to test connection to
struct Endpoint {
    host: String,
    ports: Vec<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_disabled: Option<bool>,
}

#[derive(Debug, Deserialize)]
// Represents a list of endpoint configurations
struct ReachabilityConfig {
    endpoints: Vec<Endpoint>,
}

impl ReachabilityConfig {
    // Loads yaml from config file into the ReachabilityConfig
    fn load_from_yaml(&mut self, file_path: String) {
        // Read file into memory
        let data = fs::read_to_string(file_path).expect("Unable to read file");
        // Marshal the data into the ReachabilityConfig struct
        let e = serde_yaml::from_str::<ReachabilityConfig>(&data).unwrap();
        self.endpoints = e.endpoints
    }
  

    // Performs connectivity test for each endpoint
    fn test_endpoints(self, timeout: Duration) {
        let mut target_urls: Vec<(String, bool)> = Vec::new();

        for e in self.endpoints.iter() {
            for p in e.ports.iter() {
                let target_url = match p {
                    80 => format!("http://{}:{}", e.host, p.to_string()),
                    443 => format!("https://{}:{}", e.host, p.to_string()),
                    22 => format!("ssh://{}:{}", e.host, p.to_string()),
                    _ => format!("http://{}:{}", e.host, p.to_string()),
                };

                let tls_disabled = match e.tls_disabled {
                    Some(t) => t,
                    None => false,
                };
                
                target_urls.push((target_url, tls_disabled));
            }
        }

        let handle = thread::spawn(move || {
            for t in target_urls.iter() {
                validate_reachability(&t.0, t.1, timeout);
            }
        });
        handle.join().unwrap();
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Timeout of each egress url test
    #[clap(short, long, default_value_t = 2)]
    timeout: u64,

    /// Path to configuration file
    #[clap(short, long, default_value = "config.yaml")]
    config: String,
}

fn main() {
    let args = Args::parse();

    let timeout = Duration::from_secs(args.timeout);

    // Init new ReachabilityConfig
    let mut reachability_config = ReachabilityConfig {
        endpoints: Vec::new(),
    };

    reachability_config.load_from_yaml(String::from(args.config));

    reachability_config.test_endpoints(timeout);
}

fn validate_reachability(target_url: &String, tls_disabled: bool, timeout: std::time::Duration) {  

    // Setup the client builder with timoout provided
    let mut client_builder = reqwest::Client::builder().timeout(timeout);
    if tls_disabled {
            client_builder = client_builder.danger_accept_invalid_certs(true);
    }

    let client = client_builder.build().unwrap();

    // Build the request URL with protocol based on the port

    let request = client.get(target_url).build().unwrap();

    println!("{}", target_url);
}
