use napi_derive::napi;

#[napi(object)]
pub struct TlsConfig {
	pub cert: String,
	pub ket: String
}