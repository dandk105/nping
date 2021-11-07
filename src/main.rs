use std::net::Ipv4Addr;
use pnet::packet::icmp::{IcmpPacket};
use structopt::StructOpt;


#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short, long)]
    debug: bool,

    target_ip: String
}

fn main() {
    let args = Cli::from_args();
    if let true = args.debug {
        let target_name: Ipv4Addr = args.target_ip.parse().unwrap();
        println!("Target_Name: {:?}",target_name)
    } else {
        let demo: Ipv4Addr = args.target_ip.parse().unwrap();
        let icmp_packet = IcmpPacket::new(demo);
    }
}