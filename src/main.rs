use std::net::{IpAddr};
use std::fmt;
use std::error;
use pnet::datalink::NetworkInterface;

use pnet::packet::icmp::{IcmpPacket,IcmpTypes,echo_reply,echo_request};
use structopt::StructOpt;


#[derive(Debug,Clone)]
struct UnusedIpv6addr;

impl fmt::Display for UnusedIpv6addr {
    fn fmt(&self, f: &mut fmt::Formatter ) -> fmt::Result {
        match *self {
            UnusedIpv6addr => f.write_str("Sorry This application not have allowed ipv6addr type yet \n")
        }
    }
}

// error型実装
// LOW
impl error::Error for UnusedIpv6addr {}

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short, long)]
    debug: bool,

    target_ip: String
}

fn check_ipv4(a: &String) -> bool {
    use std::str::FromStr;
    // IPv4 または、IPv6どちらにも、関数の引数:aが合致しない場合は、パニックを起こして終了する
    let ip: IpAddr = IpAddr::from_str(&a).unwrap_or_else(|_| panic!("Check your inputed types. Is it a IP type ?"));
    // パニックを起こして終了しなかった場合は、IPv4または、v6のどちらかに限定できる。
    if let true = ip.is_ipv4() {
        true
    } else {
        false
    }
}

fn handle_icmpv4(interface_name: &str, source: IpAddr, destination: IpAddr, packet: &[u8]) {
    // icmp_packetをpacket引数から作成
    let icmp_packet = IcmpPacket::new(packet);
    // icmp_packet変数が、何らかの型であった場合match式で評価する
    if let Some(icmp_packet) = icmp_packet {
        match icmp_packet.get_icmp_type() {
            // get_icmp_type()で得られた型がIcmpTypes::EchoReplyだった場合
            IcmpTypes::EchoReply => {
                let echo_reply_packet = echo_reply::EchoReplyPacket::new(packet).unwrap();
                println!(
                    "[{}]: ICMP echo reply {} -> {} (icmp_seq={:?}, id={:?})",
                    interface_name,
                    source,
                    destination,
                    echo_reply_packet.get_sequence_number(),
                    echo_reply_packet.get_identifier()
                );
            }
            // get_icmp_type()で得られた型がIcmpTypes::EchoRequestだった場合
            IcmpTypes::EchoRequest => {
                let echo_request_packet = echo_request::EchoRequestPacket::new(packet).unwrap();
                println!()
            }
            // get_icmp_type()で得られた型がEchoReplyでも、EchoRequestでもなかった場合
            _ => {}
        }
    // Some式で評価されなかった場合はicmp packet以外の型になるので、パニックを起こして終了する
    } else {
        panic!("this packet type is not icmp")
    }
}

fn main() {
    use pnet::datalink;
    use pnet::datalink::Channel::Ethernet;

    let args = Cli::from_args();
    let target: String = args.target_ip.parse().unwrap();

    if let true = check_ipv4(&target) {
        // コマンドラインから受け取った変数がcheck_ipv4関数を通過した際に、v4かv6に限定できるので、
        // 最初にコマンドライン引数を引き取っているtarget変数をシャドーイングしている
        let target = &target;
    } else {
        panic!("Sorry this project not accept ipv6addr type yet.")
    };

    /*
    // open_datalink_channelの戻り値を考える必要がある。-> HIGH
    // 現状だと、イーサネットに返却されたタプルを返す仕様にしているが、お手本の模倣をしているだけで、理解にまで及んでいない
    fn open_datalink_channel(i: &NetworkInterface) -> (tx, rx) {
        let (_, mut rtx) = match datalink::channel(i, Default::default()) {
            Ok(Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => print!("this interface dont accept"),
            Err(e) => panic!("{:?}",e)
        };
        Ok((_, mut rtx))
    }

    if let true = args.debug {
        println!("Target_Name: {:?}", ipv4addr)
    } else {
        // 宛先の指定使い回しすぎな気がする。
        // 今のところ、ipv4addrは、
        // empty -> &str -> &[u8]という動きをしているから、どこかで変数を分けるなり、したほうが良さそう。
        let ipv4addr: &[u8]  = ipv4addr.as_ref();
        handle_icmpv4(i, s, d, ipv4addr)
    }
    */
}

mod tests {
    use super::*;

    #[test]
    fn it_works_with_ipv4() {
        // IPv4の形の文字列を作成
        // 最大と最小,具体的なドメイン名
        // 8.8.8.8, 1.1.1.1
        let ipv4_255 = String::from("255.255.255.255");
        let ipv4_0 = String::from("0.0.0.0");
        let google_domain = String::from("8.8.8.8");
        let cloudflare_domain = String::from("1.1.1.1");
    
        assert_eq!(true,check_ipv4(&ipv4_255));
        assert_eq!(true,check_ipv4(&ipv4_0));
        assert_eq!(true,check_ipv4(&google_domain));
        assert_eq!(true,check_ipv4(&cloudflare_domain));
    }

    #[test]
    fn it_not_works_with_ipv6() {
        let ipv6 = String::from("::1");
        // should_panicを導入しているから、
        // panicが発生して欲しいのだけれど、その方法がよくわかっていない
        assert_eq!(false,check_ipv4(&ipv6));
        assert_ne!(true,check_ipv4((&ipv6)))
        // エラーメッセージにUnusedIpv6addrエラーは、std::cmp::PartialEqの実装していないから、
        // assert_eqで囲っても意味ないよとエラーが出たので、このエラーが発生しないように、
        // カスタムエラーを定義しなければいけない
    }

    #[test]
    #[should_panic]
    fn it_not_work_with_expect_ipaddr() {
        let num = String::from("1234567890");
        let float = String::from("2.143265");

        check_ipv4(&num);
        check_ipv4(&float);
    }
}
/*
    #[test]
    fn handle_function_work() {
        handle_icmpv4(interface_name: &str, source: IpAddr, destination: IpAddr, packet: &[u8])
    }
    
}
*/