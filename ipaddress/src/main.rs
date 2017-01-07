extern crate ipaddress;

fn main() {
    let address = ipaddress::client::fetch();

    println!("Your IP Address {:?}", address);
}
