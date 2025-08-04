
use napi_derive::napi;
use napi::bindgen_prelude::Function;

#[napi(object)]
pub struct HttpxServerOptions {
	/// TLS Option for HTTP/2 and HTTP/3
	pub tls: Option<crate::common::tls::TlsConfig>,
	pub enable_http2: Option<bool>,
	pub enable_http3: Option<bool>,
}

#[napi(string_enum)]
pub enum HttpxServerEvents {
	/// Emitted each time a request with an HTTP `Expect: 100-continue` is received.
	/// 
	/// If this event is not listened for, the server will automatically respond with a `100 Continue` as appropriate.
	/// 
	/// https://nodejs.org/api/http.html#event-checkcontinue
	CheckContinue,

	/// Emitted each time a request with an HTTP `Expect` header is received, where the value is not `100-continue`.
	/// 
	/// If this event is not listened for, the server will automatically respond with a `417 Expectation` Failed as appropriate.
	/// 
	/// https://nodejs.org/api/http.html#event-checkexpectation
	CheckExpectation,

	/// If a client connection emits an 'error' event, it will be forwarded here.
	/// 
	/// https://nodejs.org/api/http.html#event-clienterror
	ClientError,

	/// Emitted when the server closes.
	/// 
	/// https://nodejs.org/api/http.html#event-close_1
	Close,

	/// Emitted each time a client requests an HTTP `CONNECT` method.
	/// 
	/// https://nodejs.org/api/http.html#event-connect_1
	Connect,

	/// This event is emitted when a new TCP stream is established.
	/// 
	/// https://nodejs.org/api/http.html#event-connection
	Connection,

	/// When the number of requests on a socket reaches the threshold of `max_requests_per_socket`, the server will drop new requests and emit 'dropRequest' event instead, then send `503` to client.
	/// 
	/// https://nodejs.org/api/http.html#event-droprequest
	DropRequest,

	/// Emitted each time there is a request. There may be multiple requests per connection (in the case of HTTP Keep-Alive connections).
	/// 
	/// https://nodejs.org/api/http.html#event-request
	Request,

	/// Emitted each time a client requests an HTTP upgrade. Listening to this event is optional and clients cannot insist on a protocol change.
	/// 
	/// https://nodejs.org/api/http.html#event-upgrade_1
	Upgrade
}

#[napi(string_enum)]
pub enum ConnectionProtocols {
	All,
	Http1,
	Http2,
	Http3,
	WebSocket,
	WebTransport,
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

	/// Monitoring events.
	#[napi]
	pub fn on(event: HttpxServerEvents, callback: Function) {}

	/// Delete listening
	#[napi]
	pub fn off(id: u32) {}

	/// Listen for TCP and QUIC connections at a certain address.
	/// 
	/// https://nodejs.org/api/http.html#serverlisten
	#[napi]
	pub fn listen() {}

	/// Attempt to shut down the server.
	/// 
	/// When closing successfully, call callback.
	/// 
	/// https://nodejs.org/api/http.html#serverclosecallback
	#[napi]
	pub fn close(callback: Option<Function>) {}

	/// Close all connections already connected to this server.
	/// 
	/// Default closure of all protocol connections.
	/// 
	/// https://nodejs.org/api/http.html#servercloseallconnections
	#[napi(js_name = "closeAllConnections")]
	pub fn close_all_connections(protocols: Option<ConnectionProtocols>) {}

	/// Close all connections that have been established but have not completed protocol handover.
	/// 
	/// https://nodejs.org/api/http.html#servercloseidleconnections
	#[napi(js_name = "closeIdleConnections")]
	pub fn close_idle_connections() {}

	/* ------ Getter ------ */

	/// Is listening for connections.
	/// 
	/// https://nodejs.org/api/http.html#serverlistening
	#[napi(getter, js_name = "listening")]
	pub fn get_listening() {}

	/// Limit the amount of time the parser will wait to receive the complete HTTP headers.
	/// 
	/// https://nodejs.org/api/http.html#serverheaderstimeout
	#[napi(getter, js_name = "headersTimeout")]
	pub fn get_headers_timeout() {}

	/// Limits maximum incoming headers count. If set to 0, no limit will be applied.
	/// 
	/// https://nodejs.org/api/http.html#servermaxheaderscount
	#[napi(getter, js_name = "maxHeadersCount")]
	pub fn get_max_headers_count() {}

	/// Sets the timeout value in milliseconds for receiving the entire request from the client.
	/// 
	/// https://nodejs.org/api/http.html#serverrequesttimeout
	#[napi(getter, js_name = "requestTimeout")]
	pub fn get_request_timeout() {}

	/// The maximum number of requests socket can handle before closing keep alive connection.
	/// 
	/// https://nodejs.org/api/http.html#servermaxrequestspersocket
	#[napi(getter, js_name = "maxRequestsPerSocket")]
	pub fn get_max_requests_per_socket() {}

	/// The number of milliseconds of inactivity before a socket is presumed to have timed out.
	/// 
	/// https://nodejs.org/api/http.html#servertimeout
	#[napi(getter, js_name = "timeout")]
	pub fn get_timeout() {}

	/// The number of milliseconds of inactivity a server needs to wait for additional incoming data, after it has finished writing the last response, before a socket will be destroyed.
	/// 
	/// https://nodejs.org/api/http.html#serverkeepalivetimeout
	#[napi(getter, js_name = "keepAliveTimeout")]
	pub fn get_keep_alive_timeout() {}

	/* ------ Setter ------ */

	/// Limit the amount of time the parser will wait to receive the complete HTTP headers.
	/// 
	/// https://nodejs.org/api/http.html#serverheaderstimeout
	#[napi(setter, js_name = "headersTimeout")]
	pub fn set_headers_timeout(value: u32) {}

	/// Limits maximum incoming headers count. If set to 0, no limit will be applied.
	/// 
	/// https://nodejs.org/api/http.html#servermaxheaderscount
	#[napi(setter, js_name = "maxHeadersCount")]
	pub fn set_max_headers_count(value: u32) {}

	/// Sets the timeout value in milliseconds for receiving the entire request from the client.
	/// 
	/// https://nodejs.org/api/http.html#serverrequesttimeout
	#[napi(setter, js_name = "requestTimeout")]
	pub fn set_request_timeout(value: u32) {}

	/// The maximum number of requests socket can handle before closing keep alive connection.
	/// 
	/// https://nodejs.org/api/http.html#servermaxrequestspersocket
	#[napi(setter, js_name = "maxRequestsPerSocket")]
	pub fn set_max_requests_per_socket(value: u32) {}

	/// The number of milliseconds of inactivity before a socket is presumed to have timed out.
	/// 
	/// https://nodejs.org/api/http.html#servertimeout
	#[napi(setter, js_name = "timeout")]
	pub fn set_timeout(value: u32) {}

	/// The number of milliseconds of inactivity a server needs to wait for additional incoming data, after it has finished writing the last response, before a socket will be destroyed.
	/// 
	/// https://nodejs.org/api/http.html#serverkeepalivetimeout
	#[napi(setter, js_name = "keepAliveTimeout")]
	pub fn set_keep_alive_timeout(value: u32) {}
}