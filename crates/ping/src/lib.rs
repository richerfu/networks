use std::time::Instant;
use std::{net::SocketAddr, sync::Arc};

use config::{Address, Config};
use result::{PingResponse, Status};

use napi_derive_ohos::napi;
use napi_ohos::Error;
use pnet::packet::icmp::echo_reply::EchoReplyPacket;
use pnet::packet::icmp::echo_request::{IcmpCodes, MutableEchoRequestPacket};
use pnet::packet::icmp::IcmpTypes;
use pnet::packet::{util, MutablePacket, Packet};
use socket2::{Domain, Protocol, Socket, Type};

mod config;
mod result;

#[napi(object)]
pub struct PingOption {
    /// ping地址
    pub addr: String,
    /// 请求次数
    pub count: Option<u32>,
    /// 超时时间 单位：ms
    pub timeout: Option<u32>,
    /// 启动线程数
    pub thread: Option<u32>,
}

#[napi]
pub struct Ping {
    config: Config,
    dest: SocketAddr,
    socket: Arc<Socket>,
}

#[napi]
impl Ping {
    #[napi(constructor)]
    pub fn new(option: PingOption) -> napi::Result<Ping> {
        let host = option.addr.as_str();
        let address = Address::parse(host)?;
        let config = Config {
            addr: address,
            count: option.count.unwrap_or(10),
            timeout: option.timeout.unwrap_or(5000),
            thread: option.thread.unwrap_or(5),
        };
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::ICMPV4))?;
        let dest = SocketAddr::new(config.addr.ip, 0);
        Ok(Self {
            config,
            socket: Arc::new(socket),
            dest,
        })
    }

    #[napi]
    pub fn ping(&self) -> napi::Result<Vec<PingResponse>> {
        // create icmp request packet
        let mut buf = vec![0; 64];
        let mut icmp = MutableEchoRequestPacket::new(&mut buf[..])
            .ok_or(Error::from_reason("Create ICMP packet failed"))?;
        icmp.set_icmp_type(IcmpTypes::EchoRequest);
        icmp.set_icmp_code(IcmpCodes::NoCode);
        icmp.set_sequence_number(1);
        icmp.set_identifier(2134);
        icmp.set_checksum(util::checksum(icmp.packet(), 1));

        let start = Instant::now();

        // send request
        self.socket.send_to(icmp.packet_mut(), &self.dest.into())?;

        // handle recv
        let mut mem_buf =
            unsafe { &mut *(buf.as_mut_slice() as *mut [u8] as *mut [std::mem::MaybeUninit<u8>]) };
        let (_size, _) = self.socket.recv_from(&mut mem_buf)?;

        let duration = start.elapsed().as_micros() as f64 / 1000.0;
        let _reply = EchoReplyPacket::new(&buf)
            .ok_or(Error::from_reason("Get ICMP response packet failed"))?;
        let response = PingResponse {
            status: Status::Success,
            ttl: duration,
            dest: self.config.addr.ip.to_string(),
        };
        let result: Vec<PingResponse> = vec![response];
        Ok(result)
    }
}
