<h1 align="center">
	Axolotl
</h1>

<p align="center">
	Using the complete web protocol in a <a href="https://nodejs.org/api/n-api.html">N-API</a> env.
</h1>

## Why Axolotl
We have stitched and bound many existing Rust libraries using N-API to enable developers to use robust web protocols.

## Features
+ **Cross-Runtime.** Due to the compatibility of N-API with mainstream JS runtime ([Node.js][Node], [Bun][Bun], and [Deno][Deno]), it is fully available at runtime.
+ **Easy-to-Use.** Quickly create web protocol services through a simple API.
+ **High-Performance.** Rust + Tokio ensures maximum efficiency in utilizing performance.

| Protocols	| Libraries	| API Design References	|
| ---		| ---		| ---					|
| HTTP/1、HTTP/2 | [hyper][Hyper] | [node:http][Node:HTTP] |
| HTTP/3 | [h3][H3] | [node:http][Node:HTTP] |
| WebSocket | [hyper-tungstenite][Hyper-WS] | [npm:ws][Npm:WS] |
| WebTransport | [wtransport][WTransport] | [npm:@fails-components/webtransport][Npm:WebTransport] |

## Plan
+ [ ] Implement Client.
+ [ ] Support [WebRTC](https://github.com/webrtc-rs/webrtc).

[Node]: https://github.com/nodejs/node
[Bun]: https://github.com/oven-sh/bun
[Deno]: https://github.com/denoland/deno

[Hyper]: https://github.com/hyperium/hyper
[Hyper-WS]: https://github.com/de-vri-es/hyper-tungstenite-rs
[H3]: https://github.com/hyperium/h3
[WTransport]: https://github.com/BiagioFesta/wtransport

[Node:HTTP]: https://nodejs.org/api/http.html#httpcreateserveroptions-requestlistener
[Npm:WS]: https://github.com/websockets/ws
[Npm:WebTransport]: https://github.com/fails-components/webtransport