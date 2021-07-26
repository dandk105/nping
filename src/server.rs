use std::net::{Ipv4Addr, SocketAddrV4,TcpListener,TcpStream};

fn treat_client(stream: TcpStream) {
    println!("new Client: {:?}", stream);
}

fn main() -> std::io::Result<()> {
    let _localhost = Ipv4Addr::LOCALHOST;
    let sock = SocketAddrV4::new(_localhost,3500);
    let addr = TcpListener::bind(sock)?;
    println!("Success a bind socket {:?}",addr);

    for stream in addr.incoming(){
        match stream {
            Ok(stream) => {
                println!("coming a new client");
                treat_client(stream);
            }
            Err(_) => {println!("failed connecting to client")}
        }
    };
    Ok(())
}