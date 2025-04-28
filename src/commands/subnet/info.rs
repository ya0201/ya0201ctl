extern crate ipnet;
use ipnet::IpNet;
use serde_derive::Serialize;
use std::net::IpAddr;
use std::str::FromStr;
use tabled::{builder::Builder, settings::style::HorizontalLine, settings::Style};

#[derive(Serialize)]
struct MySubnet {
    network_address: String,
    broadcast_address: String,
    cidr: String,
    hosts: Vec<IpAddr>,
    number_of_ips: usize,
}

pub fn run(cidr: String, json: bool) {
    let net = IpNet::from_str(&cidr).unwrap();

    let mysubnet = MySubnet {
        network_address: net.network().to_string(),
        broadcast_address: net.broadcast().to_string(),
        cidr,
        hosts: net.hosts().collect(),
        number_of_ips: net.hosts().count() + 2,
    };

    if json {
        let json_content =
            serde_json::to_string_pretty(&mysubnet).expect("Failed to convert to JSON");
        println!("{}", json_content);
    } else {
        let mut builder = Builder::default();
        builder.push_record(["Field", "Value"]);

        builder.push_record(["Network Address", mysubnet.network_address.as_str()]);
        builder.push_record([
            "Hosts",
            format!("{}-{}", mysubnet.hosts[0], mysubnet.hosts.last().unwrap()).as_str(),
        ]);
        builder.push_record(["Broadcast Address", mysubnet.broadcast_address.as_str()]);
        builder.push_record(["CIDR", mysubnet.cidr.as_str()]);
        builder.push_record(["Number of IPs", mysubnet.number_of_ips.to_string().as_str()]);

        let mut table = builder.build();
        let style = Style::ascii()
            .horizontals([(1, HorizontalLine::inherit(Style::ascii()))])
            .remove_horizontal();
        table.with(style);

        println!("{}", table);
    }
}
