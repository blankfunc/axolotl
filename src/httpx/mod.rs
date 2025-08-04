use napi_derive::napi;

#[napi(object)]
pub struct HttpxServerOptions {

}

#[napi]
pub struct HttpxServer {

}

#[napi]
impl HttpxServer {
	#[napi(constructor)]
	pub fn new(options: Option<HttpxServerOptions>) -> Self {
		Self {}
	}
}