# Cex_Aggregator

This Rust project is an aggregator for cryptocurrency exchange (CEX) order books. It fetches order book data from different exchanges (such as Binance and Bitstamp), combines the order books, and provides the combined top orders through a gRPC server.

----------------------------------------------
### Setting up your Project.
- Install `Rust`  [official Rust guide](https://www.rust-lang.org/tools/install)
- Install `Protobuf`  [Install protoc](https://grpc.io/docs/protoc-installation/)
- Clone the Repository.

------------------------------------------------
### How to Test.
-  To run the tests for the entire project, use `cargo test --workspace`.
- To run the tests for a specific library, navigate to the library's directory and run `cargo test`.
-  Don't forget to start the server before the test, because `grpc` lib tests will be worked with running grpc server.

-----------------------------------------------
### How to Run.

-  Make sure you have Rust and Cargo installed. If not, follow the [official Rust guide](https://www.rust-lang.org/tools/install) to install them.
-  Clone this repository to your local machine.
-  Navigate to the root of the project directory and run `cargo run --bin server` to start the server.
-  In a new terminal window, navigate to the project directory and run `cargo run --bin client` to start the client.


--------------------------------------------------
### Error Guide.
**If you got error's in the building, so you have to follow these steps**
-  We have to investigate the `build.rs`.
-  Path of the build file is `Cex_Orderbook_Agg/grpc/build.rs`.
-  Here is the code of `build.rs`
```
// APPROACH - 1
 fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/orderbook.proto")?;
    Ok(())
}

// APPROACH - 2 If found any bugs in proto
//Pass your complete proto file address here to compile.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_file = "/home/mrghost/Desktop/Rust/Aggregator/Cex_Orderbook_Agg/grpc/proto/orderbook.proto";
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .compile(&[proto_file], &["/home/mrghost/Desktop/Rust/Aggregator/Cex_Orderbook_Agg/grpc/proto"])?;

    Ok(())
}
```
-  Do Follow APPROACH-1 or 2 as per your system configuration. Here are two different build concepts one is straightforward and the second one is for path the following concept. In my Case Approach-2 is working, in this you have to update the path of your proto file. `/home/{yourusername}/Desktop/Github/Cex_Orderbook_Agg/grpc/proto/orderbook.proto`
Could you check which one works better for you?

----------------------------------------------------------------------------------------------

