extern crate hyper;
extern crate json;
use self::hyper::client::{Client};
use std::io::Read;

static IP_URL: &'static str = "http://ip.jsontest.com/";

pub fn fetch() -> String {
    let mut body = String::new();
    let client = Client::new();
    client
      .get(IP_URL)
      .send()
      .unwrap()
      .read_to_string(&mut body)
      .unwrap();

    let response = json::parse(body.as_str()).unwrap();
    return response["ip"].dump();
}
