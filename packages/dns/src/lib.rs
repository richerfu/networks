use napi::{Error, Result};
use napi_derive::napi;
use std::net::AddrParseError;
use std::str::FromStr;
use trust_dns_client::client::{Client, SyncClient};
use trust_dns_client::op::DnsResponse;
use trust_dns_client::rr::{DNSClass, Name, RData, Record, RecordType};
use trust_dns_client::udp::UdpClientConnection;

#[napi(object)]
pub struct CommonOptions {
  // DNS地址 默认使用8.8.8.8
  pub dns_server: Option<String>,
  // DNS端口 默认使用53
  pub dns_port: Option<u32>,
}

#[napi(object)]
pub struct DnsOptions {
  // ping地址
  pub addr: String,
}

#[napi(object)]
pub struct DnsResponseRecords {
  pub record: String,
}

impl DnsResponseRecords {
  pub fn new(record: String) -> Self {
    Self { record }
  }
}

#[napi]
pub fn resolve_addr(
  addr: DnsOptions,
  options: Option<CommonOptions>,
) -> Result<Vec<DnsResponseRecords>> {
  let addr_name = addr.addr.to_string();

  // 构造DNS Server参数
  let mut dns_server = "8.8.8.8";
  let mut dns_port: u32 = 53;

  if let Some(ref option) = options {
    if let Some(ref server) = option.dns_server {
      dns_server = server.as_str();
    }
    if let Some(port) = option.dns_port {
      dns_port = port;
    }
  }

  let socket_addr = format!("{}:{}", dns_server, dns_port);

  let address = socket_addr.parse().map_err(|e: AddrParseError| {
    Error::from_reason(format!("Socket Addr parse failed with: {}", e.to_string()))
  })?;
  let conn = UdpClientConnection::new(address).map_err(|e| {
    Error::from_reason(format!(
      "Create socket connection failed with: {}",
      e.to_string()
    ))
  })?;
  let client = SyncClient::new(conn);

  let name = Name::from_str(addr_name.as_str())
    .map_err(|e| Error::from_reason(format!("Addr parse failed with: {}", e.to_string())))?;

  let response: DnsResponse = client
    .query(&name, DNSClass::IN, RecordType::A)
    .map_err(|e| Error::from_reason(format!("Query dns server failed with: {}", e.to_string())))?;

  let answers: &[Record] = response.answers();

  let mut results: Vec<DnsResponseRecords> = Vec::new();

  for record in answers {
    if let Some(RData::A(ref ip)) = record.data() {
      results.push(DnsResponseRecords::new(ip.to_string()));
    }
  }
  Ok(results)
}
