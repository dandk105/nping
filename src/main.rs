use std::net::{IpAddr,Ipv4Addr};
use std::fmt;

use pnet::packet::icmp::{IcmpPacket};
use structopt::StructOpt;


#[derive(Debug)]
struct UnusedIpv6addr;

impl fmt::Display for UnusedIpv6addr {
    fn fmt(&self, f: &mut fmt::Formatter ) -> fmt::Result {
        match *self {
            UnusedIpv6addr => f.write_str("This application not have allowed ipv6addr type yet")
        }
    }
}

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short, long)]
    debug: bool,

    target_ip: String
}

fn check_ipv4(target: String) -> Result<Ipv4Addr,UnusedIpv6addr> {
    // もしtargetが、ipv4アドレス型に一致するなら、
    // Ipv4addrを返却する
    // Ipv6アドレス型に一致するなら、パニックを起こす
    use std::str::FromStr;
    // 1行下では、エラーが起きると、おそらく、UnusedIpv6addrに吸収される
    // だけど、1行下で発生される、エラーは、そこに吸収されたくないので、別の実装が必要と思われる
    let ip: IpAddr = IpAddr::from_str(&target).unwrap();
    if let true = ip.is_ipv6() {
        panic!("Sorry this application have not accept Ipv6addr yet")
    } else {
        let ipv4addr = Ipv4Addr::from_str(&target).unwrap();
        Ok(ipv4addr)
    }
}

fn main() {
    let args = Cli::from_args();
    // 引数が不明な型だった場合に、パニックを起こす
    // trget_ipの型が示すのが何かわかっていないから混乱しているのでは？
    // struct_optで示されているのは、target_ipの型
    let target: String = args.target_ip.parse().unwrap_or_else(|_| panic!("the inputed type dont accept in this app"));

    // ipv6の型だった場合にエラーを起こす
    // ipv4だとresult返す
    let ipv4addr = check_ipv4(target).unwrap();

    if let true = args.debug {
        println!("Target_Name: {:?}", ipv4addr)
    } else {
        let ipv4addr: &[u8]  = ipv4addr;
        let icmp_packet = IcmpPacket::new(ipv4addr);
    }
}