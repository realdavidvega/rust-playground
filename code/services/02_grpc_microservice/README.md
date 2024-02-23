# gRPC Microservice

Example of Microservice architecture using:

* [Tonic](https://docs.rs/tonic/0.3.1/tonic/): a gRPC implementation.
* [Prost](https://github.com/tokio-rs/prost): protocol buffer implementation.
* [Tokio](https://tokio.rs/): an asynchronous runtime for network apps.

## How to run

Server

```bash
cargo run --bin payments-server
```

Client

```bash
cargo run --bin payments-client
```
