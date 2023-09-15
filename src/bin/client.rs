use lazy_static::lazy_static;
use mini_redis::{FilterLayer, LogLayer};
use std::{env, net::SocketAddr, sync::Arc};
use volo::FastStr;
use volo_gen::volo::example::{GetItemRequest, RedisCommand};

lazy_static! {
    static ref CLIENT: volo_gen::volo::example::ItemServiceClient = {
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        volo_gen::volo::example::ItemServiceClientBuilder::new("volo-example")
            .layer_outer(LogLayer)
            .layer_outer(FilterLayer)
            .address(addr)
            .build()
    };
}

#[volo::main]
async fn main() {
    tracing_subscriber::fmt::init();
        let args: Vec<String> = env::args().collect();
        let _req = match args[1].to_lowercase().as_str() {
            "get" => GetItemRequest {
                key: Some(FastStr::from(Arc::new(args[2].clone()))),
                value: None,
                command: RedisCommand::Get,
            },
            "set" => GetItemRequest {
                key: Some(FastStr::from(Arc::new(args[2].clone()))),
                value: Some(FastStr::from(Arc::new(args[3].clone()))),
                command: RedisCommand::Set,
            },
            "del" => GetItemRequest {
                key: Some(FastStr::from(Arc::new(args[2].clone()))),
                value: None,
                command: RedisCommand::Del,
            },
            "ping" => GetItemRequest {
                key: None,
                value: None,
                command: RedisCommand::Ping,
            },
            "subscribe" => GetItemRequest {
                key: Some(FastStr::from(Arc::new(args[2].clone()))),
                value: None,
                command: RedisCommand::Subscribe,
            },
            "publish" => GetItemRequest {
                key: Some(FastStr::from(Arc::new(args[2].clone()))),
                value: Some(FastStr::from(Arc::new(args[3].clone()))),
                command: RedisCommand::Publish,
            },
            "exit" => {
                println!("Bye!");
                return;
            }
            _ => {
                panic!("unknown command");
            }
        };

        let resp = CLIENT.get_item(_req).await;
        match resp {
            Ok(info) => {
                if info.flag {
                    println!("Success {:?}", info.res.unwrap());
                } else {
                    println!("Error {:?}", info.res.unwrap());
                }
            }
            Err(e) => {
                tracing::error!("{:?}", e);
            }
        }
}
