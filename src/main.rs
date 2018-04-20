extern crate reqwest;
extern crate serde_json;

use std::env;
use serde_json::Value;

fn get_text(website: &str) {
    let text = reqwest::get(website).unwrap().text().unwrap();
    let v: Value = serde_json::from_str(&text).unwrap();
    println!("hello {}", v["hello"].as_str().unwrap());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "install" => get_text("https://api.myjson.com/bins/1gq93v"),
        _ => panic!("FAIL!"),
    }
}
