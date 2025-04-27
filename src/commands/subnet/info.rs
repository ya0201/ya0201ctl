extern crate ipnet;
use ipnet::IpNet;
use std::str::FromStr;
pub fn run(cidr: String, json: bool) {
    // println!("Subnet info: {}", cidr);
    if json {
        println!("json flag specified");
    } else {
        let net = IpNet::from_str(&cidr).unwrap();
        println!("{} hostmask = {}", net, net.hostmask());
        println!("{} netmask = {}", net, net.netmask());
    }
}
