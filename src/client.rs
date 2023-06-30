pub mod kvstore {
    // The string specified here must match the proto package name
    mrpc::include_proto!("kvstore");
    // include!("../../../mrpc/src/codegen.rs");
}

use kvstore::kvstore_client::KvstoreClient;
use kvstore::GetParams;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = KvstoreClient::connect("localhost:5000")?;
    let req = GetParams { key: "mRPC".into() };
    let reply = smol::block_on(client.get(req))?;
    println!("reply: {}", &reply.key);
    Ok(())
}
