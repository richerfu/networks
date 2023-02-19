use napi_derive::napi;

#[napi(object)]
pub struct PingOptions {
  // ping地址
  pub addr: String,
  // 请求次数
  pub count: u32,
  // 超时时间 单位：ms
  pub timeout: u32,
}

#[napi]
pub fn create_ping_instance() -> PingOptions {
  PingOptions { addr: String::from("12"), count: 1, timeout: 3000 }
}