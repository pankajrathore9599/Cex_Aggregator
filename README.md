# Cex_Aggregator

Cex_Aggregator is a Rust project that serves as an aggregator for cryptocurrency exchange (CEX) order books. The application fetches order book data from various exchanges such as Binance and Bitstamp, combines these order books, and provides the combined top orders via a gRPC server.
---------------------------------
## Project Architecture
```
├── cex_orderbook_agg
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── exchanges
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── binance.rs
│   │       ├── bitstamp.rs
│   │       └── lib.rs
│   ├── exc_orderbook
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── combine_orderbook.rs
│   │       └── lib.rs
│   ├── grpc
│   │   ├── build.rs
│   │   ├── Cargo.toml
│   │   ├── proto
│   │   │   └── orderbook.proto
│   │   └── src
│   │       ├── client.rs
│   │       ├── lib.rs
│   │       └── server.rs
│   └── src
│       └── main.rs
├── frontend
│   ├── package.json
│   └── src
│       ├── index.js
│       ├── components
│       └── ...
├── LICENSE
└── README.md

```

---------------------------------
## Setup

- Install `Rust` following the [official Rust guide](https://www.rust-lang.org/tools/install).
- Install `Protobuf` using the [Protoc Installation Guide](https://grpc.io/docs/protoc-installation/).
- Clone this repository.
---------------------------------------------------------------------

## Testing

- Navigate to the `cex_orderbook_agg` directory.
- To run tests for the entire project, execute `cargo test --workspace`.
- To run tests for a specific library, navigate to the library's directory and run `cargo test`.
- To make the gRPC server available for testing, use the command `cargo run --package grpc --bin server`.
- Remember to start the server before running tests as the `grpc` library tests require a running gRPC server.
-------------------------------------------------------------------
## Execution

- Clone this repository to your local machine.
- Ensure that Rust and Cargo are installed. If not, follow the [official Rust guide](https://www.rust-lang.org/tools/install).
- Build the project using `cargo build`.
- Navigate to the root of the project directory, `cex_orderbook_agg`, and execute `cargo run` to receive the combined order book of centralized exchanges, including the top 10 "Asks" and "Bids" with "Spread".
- Start the gRPC server using the command `cargo run --package grpc --bin server`. The server will provide live data at `0.0.0.0:50051`.
- Start the Rust-based client to check live feeds from websockets. In another terminal from the root project directory (`cex_orderbook_agg`), execute the command `cargo run --package grpc --bin client`.

---------------------------------------------------------------------
## Frontend Setup

- Navigate to the **Frontend** folder in the repository.
- Install npm packages with `npm install`.
- Ensure your Rust server is running as per the steps above.
- To start the Node.js client and receive live feeds from websockets, run `node src/client.js`.

---------------------------------------------------------------------
## Troubleshooting

If you encounter build errors, follow these steps:
- Check the `build.rs` file.
- The path of the build file is `Cex_Orderbook_Agg/grpc/build.rs`.
- Here is the code of `build.rs`:

```rust
// APPROACH - 1
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/orderbook.proto")?;
    Ok(())
}

// APPROACH - 2 If found any bugs in proto
// Specify the complete proto file address here to compile.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_file = "/home/mrghost/Desktop/Rust/Aggregator/Cex_Orderbook_Agg/grpc/proto/orderbook.proto";
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .compile(&[proto_file], &["/home/mrghost/Desktop/Rust/Aggregator/Cex_Orderbook_Agg/grpc/proto"])?;

    Ok(())
}
```
- Choose either APPROACH-1 or 2 based on your system configuration. The first approach is straightforward, while the second considers your specific file path. In my case, APPROACH-1 worked. If using APPROACH-2, update the path of your proto file to `/home/{yourusername}/Desktop/Github/Cex_Orderbook_Agg/grpc/proto/orderbook.proto`.
