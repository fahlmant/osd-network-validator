extern crate serde_yaml;
extern crate clap;

use std::fs;
use std::time::Duration;
use clap::{Parser};

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
        for e in self.endpoints.iter() {
            for p in e.ports.iter() {
                // Validate that the endpoint is reachable for each host:port combination
                validate_reachability(&e.host, p, e.tls_disabled, timeout);
            }
        }
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long,default_value_t = 2)]
    timeout: u64,

    /// Number of times to greet
    #[clap(short, long, default_value = "config.yaml")]
    config: String,
}

fn main() {

    let args = Args::parse();

    let timeout = Duration::from_secs(args.timeout);

    // Init new ReachabilityConfigl
    let mut reachability_config = ReachabilityConfig {
        endpoints: Vec::new(),
    };

    reachability_config.load_from_yaml(String::from(args.config));

    reachability_config.test_endpoints(timeout);
}

// Attempts to establish a connection and communicate to the provided host:port combinationk
fn validate_reachability(host: &String, port: &i32, tls_disabled: Option<bool>, timeout: Duration) {

    // Setup the client builder with timoout provided
    let mut client_builder = reqwest::Client::builder().timeout(timeout);

    // If tls is explicitly disabled, then add option to accept invalid certs to client builder
    // Do nothing if tls_disabled is false or missing
    match tls_disabled {
        Some(t) => if t {client_builder = client_builder.danger_accept_invalid_certs(true);}
        None => {},
    }

    let client = client_builder.build();
    
    match port {
        80 => println!("{}:{} {:?}", host, port,tls_disabled),
        443 => println!("{}:{} {:?}", host, port,tls_disabled),
        22 => println!("{}:{} {:?}", host, port,tls_disabled),
        _ => println!("{}:{} {:?}", host, port,tls_disabled),
    }
}
