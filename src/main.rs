extern crate serde_yaml;

#[macro_use]
extern crate serde_derive;


struct ReachabilityConfig {
    Endpoints: Vec<Endpoint>
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
struct Endpoint {

    host: String,
    ports: Vec<i32>,
    tls_disabled: bool,
}

fn main() {

    let endpoint = Endpoint{
        host: String::from("quay.io"),
        ports: vec![80,443],
        tls_disabled: false,
    };

    let yaml = serde_yaml::to_string(&endpoint);   
    println!("{}", yaml.unwrap()) 
    
}