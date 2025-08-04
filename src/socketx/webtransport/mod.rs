use napi_derive::napi;

#[napi]
pub struct TransportServer {

}

#[napi]
impl TransportServer {
	#[napi(constructor)]
	pub fn new() -> Self {
		Self {}
	}
}