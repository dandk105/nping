use std::net::{IpAddr,Ipv4Addr};
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
    // 引数が不明な型だった場合に、パニックを起こす
    let ipaddr = args.target_ip.parse().unwrap_or_else(|_| panic!("the inputed type dont accept in this app"));

    // ipv6の型だった場合にエラーを起こす
    if let true = Ipv4Addr::from_str(ipaddr).is_ipv6() {
        panic!("Sorry this application have not accept ipv6addr type yet")
    }

    if let true = args.debug {
        let target_name: IpAddr = ipaddr;
        println!("Target_Name: {:?}",target_name)
    } else {
        // どこで判断して、IpAddr型だと判断しているんだろうか？
        let ipv4addr: &[u8]  = ipaddr.as_bytes();
        let icmp_packet = IcmpPacket::new(ipv4addr);
    }
}