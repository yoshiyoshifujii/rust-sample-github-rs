use std::env;

use github_rs::client::{Executor, Github};
use github_rs::errors::Error;
use github_rs::{HeaderMap, StatusCode};
use serde_json::Value;

fn main() {
    let args: Vec<String> = env::args().collect();
    let api_token: &String = &args[1];
    let client = Github::new(api_token).unwrap();
    let pulls = client
        .get()
        .repos()
        .owner("chatwork")
        .repo("sagrada-server")
        .pulls()
        .execute::<Value>();
    results(pulls);
}

fn results(pulls: Result<(HeaderMap, StatusCode, Option<Value>), Error>) {
    match pulls {
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
