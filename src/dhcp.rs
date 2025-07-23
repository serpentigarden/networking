use std::io::Error;
use std::net::{Ipv4Addr, UdpSocket};

pub fn allocate_new_host() -> Result<(), Error> {
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 68))?;
    socket.set_broadcast(true)?;

    let mut buf = [0; 576];
    let init = DhcpMsg::client_init([1, 2, 3, 4]);
    init.print();

    let msg_size = init.to_bytes(&mut buf);

    println!("Sending message of size {}", msg_size);
    let amt_sent = socket.send_to(&buf[0..msg_size], (Ipv4Addr::BROADCAST, 67))?;
    println!("Sent {} bytes", amt_sent);

    let mut recv_buf = [0; 576];
    let (amt_recv, from_addr) = socket.recv_from(&mut recv_buf)?;
    println!("Received {} bytes from {}", amt_recv, from_addr);

    let response = DhcpMsg::from_bytes(amt_recv, &recv_buf);
    response.print();
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
#[derive(Debug)]
pub struct DhcpMsg {
    op: u8,
    htype: u8,
    hlen: u8,
    hops: u8,
    xid: [u8; 4],
    secs: [u8; 2],
    flags: [u8; 2],
    ciaddr: [u8; 4],
    yiaddr: [u8; 4],
    siaddr: [u8; 4],
    giaddr: [u8; 4],
    chaddr: [u8; 16],
    sname: [u8; 64],
    file: [u8; 128],
    options: [u8; 340],
    options_len: usize,
}

impl DhcpMsg {
    // Up til options is 236 bytes
    pub fn client_init(xid: [u8; 4]) -> Self {
        let mut options = [0; 340];
        // magic cookie
        options[0] = 99;
        options[1] = 130;
        options[2] = 83;
        options[3] = 99;
        // end
        options[4] = 255;

        DhcpMsg {
            op: 1,
            htype: 1, // 10 mb ethernet
            hlen: 1,  // 6 bytes
            hops: 0,
            xid,
            secs: [0, 0],
            // broadcast
            flags: [1 << 7, 0],
            ciaddr: [0; 4],
            yiaddr: [0; 4],
            siaddr: [0; 4],
            giaddr: [0; 4],
            chaddr: [0; 16],
            sname: [0; 64],
            file: [0; 128],
            options: options,
            options_len: 5,
        }
    }

    pub fn from_bytes(amt: usize, buf: &[u8]) -> Self {
        let mut xid = [0; 4];
        let mut secs = [0, 0];
        let mut flags = [1 << 7, 0];
        let mut ciaddr = [0; 4];
        let mut yiaddr = [0; 4];
        let mut siaddr = [0; 4];
        let mut giaddr = [0; 4];
        let mut chaddr = [0; 16];
        let mut sname = [0; 64];
        let mut file = [0; 128];
        let mut options = [0; 340];
        let options_len = amt - 236;

        xid.copy_from_slice(&buf[4..8]);
        secs.copy_from_slice(&buf[8..10]);
        flags.copy_from_slice(&buf[10..12]);
        ciaddr.copy_from_slice(&buf[12..16]);
        yiaddr.copy_from_slice(&buf[16..20]);
        siaddr.copy_from_slice(&buf[20..24]);
        giaddr.copy_from_slice(&buf[24..28]);
        chaddr.copy_from_slice(&buf[28..44]);
        sname.copy_from_slice(&buf[44..108]);
        file.copy_from_slice(&buf[108..236]);
        options[0..options_len].copy_from_slice(&buf[236..236 + options_len]);

        DhcpMsg {
            op: buf[0],
            htype: buf[1],
            hlen: buf[2],
            hops: buf[3],
            xid,
            secs,
            flags,
            ciaddr,
            yiaddr,
            siaddr,
            giaddr,
            chaddr,
            sname,
            file,
            options,
            options_len,
        }
    }

    pub fn to_bytes(&self, buf: &mut [u8]) -> usize {
        buf[0] = self.op;
        buf[1] = self.htype;
        buf[2] = self.hlen;
        buf[3] = self.hops;

        buf[4..8].copy_from_slice(&self.xid);
        buf[8..10].copy_from_slice(&self.secs);
        buf[10..12].copy_from_slice(&self.flags);
        buf[12..16].copy_from_slice(&self.ciaddr);
        buf[16..20].copy_from_slice(&self.yiaddr);
        buf[20..24].copy_from_slice(&self.siaddr);
        buf[24..28].copy_from_slice(&self.giaddr);
        buf[28..44].copy_from_slice(&self.chaddr);
        buf[44..108].copy_from_slice(&self.sname);
        buf[108..236].copy_from_slice(&self.file);

        let options_amt = self.options_len;
        buf[236..236 + options_amt].copy_from_slice(&self.options[0..options_amt]);
        236 + options_amt
    }

    pub fn print(&self) {
        println!("{:?}", self);
    }
}
