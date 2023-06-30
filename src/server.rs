pub mod kvstore {
    // The string specified here must match the proto package name
    mrpc::include_proto!("kvstore");
}

use kvstore::kvstore_server::{Kvstore, KvstoreServer};
use kvstore::{AllItemsParams, GetParams, Item, Success};
use mrpc::{RRef, WRef};
use shm::string::String;

#[derive(Debug, Default)]
struct MyKvstore;

#[mrpc::async_trait]
impl Kvstore for MyKvstore {
    async fn get(&self, request: RRef<GetParams>) -> Result<WRef<Item>, mrpc::Status> {
        let key = String::from(&request.key);

        println!("RECIEVED GET");
        let value = String::from("helo");
        let reply = WRef::new(Item {
            key: key,
            value: value,
        });
        Ok(reply)
    }

    async fn put(&self, request: RRef<Item>) -> Result<WRef<Item>, mrpc::Status> {
        let reply = WRef::new(Item {
            key: String::from(&request.key),
            value: String::from(&request.value),
        });
        Ok(reply)
    }

    async fn get_all_items(
        &self,
        request: RRef<AllItemsParams>,
    ) -> Result<WRef<Item>, mrpc::Status> {
        let reply = WRef::new(Item {
            key: String::from("hello"),
            value: String::from("world"),
        });
        Ok(reply)
    }

    async fn delete(&self, request: RRef<GetParams>) -> Result<WRef<Success>, mrpc::Status> {
        let reply = WRef::new(Success { success: true });
        Ok(reply)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    smol::block_on(async {
        let mut server = mrpc::stub::LocalServer::bind("0.0.0.0:5000")?;

        server
            .add_service(KvstoreServer::new(MyKvstore::default()))
            .serve()
            .await?;
        println!("Server Stopped");
        Ok(())
    })
}
