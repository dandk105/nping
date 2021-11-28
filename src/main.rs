use std::net::{IpAddr};
use std::fmt;

use pnet::packet::icmp::{IcmpPacket};
use structopt::StructOpt;


#[derive(Debug)]
struct UnusedIpv6addr;

impl fmt::Display for UnusedIpv6addr {
    fn fmt(&self, f: &mut fmt::Formatter ) -> fmt::Result {
        match *self {
            UnusedIpv6addr => f.write_str("Sorry This application not have allowed ipv6addr type yet \n")
        }
    }
}

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short, long)]
    debug: bool,

    target_ip: String
}

fn check_ipv4(a: &String) -> Result<bool,UnusedIpv6addr> {
    // もしtargetが、ipv4アドレス型に一致するなら、
    // Trueを返却する
    // Ipv6アドレス型に一致するなら、パニックを起こす
    use std::str::FromStr;
    // 1行下では、エラーが起きると、おそらく、UnusedIpv6addrに吸収される
    // だけど、1行下で発生される、エラーは、そこに吸収されたくないので、別の実装が必要と思われる
    let ip: IpAddr = IpAddr::from_str(&a).unwrap_or_else(|_| panic!());
    if let true = ip.is_ipv4() {
        Ok(true)
    } else {
        Err(UnusedIpv6addr)
    }
}

fn main() {
    let args = Cli::from_args();
    let target: String = args.target_ip.parse().unwrap();

    // グローバル変数として、空の文字列を定義して、check_ipv4関数が正の場合
    // グローバル変数に、targetの参照を代入という手法をとっているが、すごいダサい。
    let mut ipv4addr = "";
    match check_ipv4(&target) {
        Ok(_) => ipv4addr = &target,
        Err(err) => print!("{}",err)
    }

    if let true = args.debug {
        println!("Target_Name: {:?}", ipv4addr)
    } else {
        let ipv4addr: &[u8]  = ipv4addr.as_ref();
        let icmp_packet = IcmpPacket::new(ipv4addr);
    }
}