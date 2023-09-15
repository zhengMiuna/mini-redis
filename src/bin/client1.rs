use lazy_static::lazy_static;
use pilota::lazy_static;
use std::net::SocketAddr;
use volo_gen::mini_redis::GetItemRequest;


lazy_static! {
    static ref CLIENT1: volo_gen::mini_redis::ItemServiceClient = {
        let addr: SocketAddr = "127.0.0.1:8081".parse().unwrap();
        volo_gen::mini_redis::ItemServiceClientBuilder::new("mini-redis")
            .address(addr)
            .build()
    };
}

#[volo::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let mut args: Vec<String> = std::env::args().collect();
    let mut req = GetItemRequest { op: " ".into(), key: " ".into(), value: " ".into() };
    let operation = args.remove(1).clone().to_lowercase().to_string();
    if operation == "set".to_string() {
        req = GetItemRequest {
            op: "deny".into(),
            key: args.remove(1).clone().into(),
            value: args.remove(1).clone().into(),
        };
    }else if operation == "get".to_string() {
        req = GetItemRequest {
            op: "get".into(),
            key: args.remove(1).clone().into(),
            value: " ".into(),
        };
    }else if operation == "del".to_string() {
        req = GetItemRequest {
            op: "del".into(),
            key: args.remove(1).clone().into(),
            value: " ".into(),
        };
    }else if operation == "ping".to_string() {
        req = GetItemRequest {
            op: "ping".into(),
            key: " ".into(),
            value: args.remove(1).clone().into(),
        };
    }else {
        println!("Invalid command");
    }
    let resp = CLIENT1.get_item(req).await;
    match resp {
        Ok(info) => {
            println!("{} finished",info.op);
        }
        Err(e) => {
            tracing::error!("{:?}",e);
        }
    }
}