use std::io::{self, Error};
use std::net;
use std::net::ToSocketAddrs;

#[derive(Debug, Clone)]
pub struct Config {
  /// ping地址
  pub addr: Address,
  /// 请求次数
  pub count: u32,
  /// 超时时间 单位：ms
  pub timeout: u32,
  /// 启动线程数
  pub thread: u32,
}

#[derive(Debug, Clone)]
pub struct Address {
  pub ip: net::IpAddr,
  pub raw: String,
}

impl Address {
  pub fn parse(host: &str) -> Result<Address, io::Error> {
    let raw = String::from(host);
    let opt = host.parse::<net::IpAddr>().ok();
    match opt {
      Some(ip) => Ok(Address { ip: ip, raw: raw }),
      None => {
        let new = format!("{}:{}", host, 0);
        let mut addrs = new.to_socket_addrs()?;
        if let Some(addr) = addrs.next() {
          Ok(Address {
            ip: addr.ip(),
            raw: raw,
          })
        } else {
          Err(Error::new(
            io::ErrorKind::Other,
            "Parse domain to ip failed",
          ))
        }
      }
    }
  }
}
