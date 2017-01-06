extern crate hyper;
use hyper::client::{Client};
use std::io::Read;
static IP_URL: &'static str = "http://ip.jsontest.com/";

fn main() {
  let mut body = String::new();
  let client = Client::new();
  client
    .get(IP_URL)
    .send()
    .unwrap()
    .read_to_string(&mut body)
    .unwrap();

  println!("{:?}", body);
}
