use std::io::Error;
use std::net::{Ipv4Addr, UdpSocket};

fn main() -> Result<(), Error> {
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 68))?;
    println!("Initialized socket");
    socket.set_broadcast(true)?;
    println!("Allow broadcast");

    // Up til options is 236 bytes
    let mut dhcp_req: [u8; 241] = [0; 241];
    // op
    dhcp_req[0] = 1;
    // htype
    dhcp_req[1] = 1;
    // hlen
    dhcp_req[2] = 1;
    // hops set to 0 by client
    // dhcp_req[3] = ..
    // xid
    dhcp_req[4] = 1;
    // secs
    // dhcp_req[8..10] = ..
    // flags. broadcast option
    dhcp_req[10] = 1 << 7;
    // ciaddr
    // yiaddr
    // siaddr
    // giaddr
    // chaddr
    // sname
    // file
    // options
    // magic cookie
    dhcp_req[236] = 99;
    dhcp_req[237] = 130;
    dhcp_req[238] = 83;
    dhcp_req[239] = 99;
    // "end option"
    dhcp_req[240] = 0xFF;

    println!("Sending message");
    let amt_sent = socket.send_to(&dhcp_req, (Ipv4Addr::BROADCAST, 67))?;
    println!("Sent {} bytes", amt_sent);
    print_dhcp_msg(241, &dhcp_req);

    let mut recv_buf: [u8; 512] = [0; 512];
    let (amt_recv, from_addr) = socket.recv_from(&mut recv_buf)?;
    println!("Received {} bytes from {}", amt_recv, from_addr);
    print_dhcp_msg(amt_recv, &recv_buf);

    Ok(())
}

/*
   0                   1                   2                   3
   0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |     op (1)    |   htype (1)   |   hlen (1)    |   hops (1)    |
   +---------------+---------------+---------------+---------------+
   |                            xid (4)                            |
   +-------------------------------+-------------------------------+
   |           secs (2)            |           flags (2)           |
   +-------------------------------+-------------------------------+
   |                          ciaddr  (4)                          |
   +---------------------------------------------------------------+
   |                          yiaddr  (4)                          |
   +---------------------------------------------------------------+
   |                          siaddr  (4)                          |
   +---------------------------------------------------------------+
   |                          giaddr  (4)                          |
   +---------------------------------------------------------------+
   |                          chaddr  (16)                         |
   +---------------------------------------------------------------+
   |                          sname   (64)                         |
   +---------------------------------------------------------------+
   |                          file    (128)                        |
   +---------------------------------------------------------------+
   |                          options (variable)                   |
   +---------------------------------------------------------------+
*/
// todo: make struct for dhcp message
fn print_dhcp_msg(amt: usize, buf: &[u8]) {
    println!(
        "op {:02X} | htype {:02X} | hlen {:02X} | hops {:02X}",
        buf[0], buf[1], buf[2], buf[3]
    );
    println!("xid {:02X?}", &buf[4..8]);
    println!("secs {:02X?} | flags {:02X?}", &buf[8..10], &buf[10..12]);
    println!("ciaddr {:02X?}", &buf[12..16]);
    println!("yiaddr {:02X?}", &buf[16..20]);
    println!("siaddr {:02X?}", &buf[20..24]);
    println!("giaddr {:02X?}", &buf[24..28]);
    println!("chaddr {:02X?}", &buf[28..44]);
    println!("sname {:02X?}", &buf[44..108]);
    println!("file {:02X?}", &buf[108..236]);
    println!("options {:02X?}", &buf[236..amt]);
}
