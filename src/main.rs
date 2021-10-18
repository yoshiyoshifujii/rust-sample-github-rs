use github_rs::client::{Executor, Github};
use serde_json::Value;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let api_token = &args[1];
    let client = Github::new(api_token).unwrap();
    let me = client.get().user().execute::<Value>();
    match me {
        Ok((headers, status, json)) => {
            println!("{:#?}", headers);
            println!("{}", status);
            if let Some(json) = json {
                println!("{}", json);
            }
        }
        Err(e) => println!("{}", e),
    }
}
