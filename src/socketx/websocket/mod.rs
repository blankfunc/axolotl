use napi_derive::napi;

#[napi]
pub struct SocketServer {

}

#[napi]
impl SocketServer {
	#[napi(constructor)]
	pub fn new() -> Self {
		Self {}
	}

	#[napi(factory)]
	pub fn with_httpx_server() -> Self {
		Self {}
	}
}