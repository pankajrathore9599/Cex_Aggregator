const grpc = require('@grpc/grpc-js');
const protoLoader = require('@grpc/proto-loader');

const PROTO_PATH = __dirname + '/../../cex_orderbook_agg/grpc/proto/orderbook.proto';

// Load the .proto file
const packageDefinition = protoLoader.loadSync(
    PROTO_PATH,
    {
        keepCase: true,
        longs: String,
        enums: String,
        defaults: true,
        oneofs: true
    }
);

// Load the package definition
const orderbookProto = grpc.loadPackageDefinition(packageDefinition).orderbook;

// Create a new client instance
const client = new orderbookProto.OrderBook('localhost:50051', grpc.credentials.createInsecure());

function getTopOrders(pair) {
    // Define the request
    const request = { top: 10, pair: pair }; // Request top 10 orders for the given pair

    // Make the request
    client.getTopOrders(request, (error, response) => {
        if (error) {
            console.log('Error:', error);
        } else {
            console.log('Top 10 Asks:', response.asks);
            console.log('Top 10 Bids:', response.bids);

            // Assuming that asks and bids are sorted
            if(response.asks.length > 0 && response.bids.length > 0) {
                // Compute the spread
                const spread = parseFloat(response.asks[0].price) - parseFloat(response.bids[0].price);
                console.log('Spread:', spread);
            }
        }
    });
}

const pair = process.argv[2] || 'ethbtc';  // get pair from command line argument or use 'ethbtc' as default
setInterval(() => getTopOrders(pair), 9000);
