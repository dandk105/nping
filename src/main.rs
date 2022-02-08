use std::error;
use std::fmt;
use std::net::IpAddr;
use std::process::exit;

use pnet::datalink;
use pnet::datalink::Channel::Ethernet;
use pnet::datalink::NetworkInterface;
use pnet::packet::icmp::{echo_reply, echo_request, IcmpPacket, IcmpTypes};

use structopt::StructOpt;

#[derive(Debug, Clone)]
struct UnusedIpv6addr;

impl fmt::Display for UnusedIpv6addr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UnusedIpv6addr => {
                f.write_str("Sorry This application not have allowed ipv6addr type yet \n")
            }
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

    target_ip: String,
}

fn get_v4_netwrok_interface(interfaces: Vec<&NetworkInterface>)->NetworkInterface{
    let v4;
    let i;
    /* for ifs in interfaces {
        v4 = ifs.ips
            .iter()
            .find(|x| x.is_ipv4());
        match v4 {}
    }
    i */
}

/*
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
*/

fn main() {
    use std::str::FromStr;

    let args = Cli::from_args();
    let target: String = args.target_ip.parse().unwrap();

    // target: &StringがIpAddr型に一致しなかった場合パニックを起こして終了。
    let target: IpAddr = IpAddr::from_str(&target)
        .unwrap_or_else(|_| panic!("Check your inputed types. Is it a IP type ?"));

    if let true = target.is_ipv4() {
        // コマンドラインから受け取った変数がis_ipv4関数を通過した際に、v4かv6に限定できるので、
        // 最初にコマンドライン引数を引き取っているtarget変数をシャドーイングしている
        let target = &target;
    } else if let true = target.is_ipv6() {
        panic!("Sorry this project not accept ipv6addr type yet.")
    } else {
        eprint!(
            "Unexpected Error.Please tell me about your inputed {:?}",
            target
        );
        exit(1)
    };

    let interfaces = datalink::interfaces();
    // struct NetworkInterfaceの中で、条件に当てはまるものを検索している
    // 条件->
    // 1.up状態である
    // 2.ループバックではない
    // 3.ipアドレスが空ではない
    //.find(|x| x.is_up() && !x.is_loopback() && !x.ips.is_empty() && x.ips.len() > 1)
    // WARN:　現状の方法で、ネットワークインターフェイスを取得すると、ipv6のものを取得して、ipv4のものは取得できない。
    let interfaces : Vec<&NetworkInterface> = interfaces
        .iter()
        .filter(|x| x.is_up() && !x.is_loopback() && !x.ips.is_empty() && x.ips.is_empty())
        .collect();

    let i = get_v4_netwrok_interface(interfaces);

    let _sender_mac = i.mac;
    let (sender, reciver) = match datalink::channel(&i, Default::default()) {
        // pnet::datalink::Channel::Ethernet(Box<dyn DataLinkSender + 'static, Global>, Box<dyn DataLinkReceiver + 'static, Global>)
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("this interface dont accept {:?}",i),
        Err(e) => panic!("{:?}", e),
    };

    //sender.build_and_send(num_packets: usize, packet_size: usize, func: &mut dyn FnMut(&mut [u8]))
    //reciver.next()
    /*
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
