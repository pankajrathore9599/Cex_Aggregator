// APPROACH - 1

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/orderbook.proto")?;
    Ok(())
}



// APPROACH - 2 If found any bugs in proto
// pass your complete proto file address here to compile.

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let proto_file = "/home/mrghost/Desktop/Rust/Cex_Aggregator/cex_orderbook_agg/grpc/proto/orderbook.proto";
//     tonic_build::configure()
//         .build_client(true)
//         .build_server(true)
//         .compile(&[proto_file], &["/home/mrghost/Desktop/Rust/Cex_Aggregator/cex_orderbook_agg/grpc/proto"])?;

//     Ok(())
// }



