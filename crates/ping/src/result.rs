use napi_derive_ohos::napi;

#[napi]
pub enum Status {
    Success,
    Fail,
    Timeout,
}

#[napi(object)]
pub struct PingResponse {
    /// 状态
    pub status: Status,
    /// 耗时
    pub ttl: f64,
    /// 请求目的ip
    pub dest: String,
}
