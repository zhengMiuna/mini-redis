use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Result},
    net::SocketAddr,
};

use mini_redis::{FilterLayer, LogLayer, S};

const AOF_FILE_PATH: &str = "aof.log";

#[volo::main]
async fn main() {
    let map = 
        load_from_aof_file(AOF_FILE_PATH).unwrap_or_else(|_| HashMap::new());

    let addr: SocketAddr = "[::]:8080".parse().unwrap();
    let addr = volo::net::Address::from(addr);

    // 创建 S 实例
    let s = S::new(map).unwrap();

    // 创建 Server，并添加 LogLayer 和 FilterLayer
    let server = volo_gen::volo::example::ItemServiceServer::new(s)
        .layer_front(LogLayer)
        .layer_front(FilterLayer);

    // 启动 Server
    server.run(addr).await.unwrap();

    // 关闭 AOF 文件
    let file = match File::open(AOF_FILE_PATH) {
        Ok(file) => Some(file),
        Err(_) => None,
    };
    if let Some(file) = file {
        file.sync_data().unwrap();
        drop(file);
    }
}


fn load_from_aof_file(file_path: &str) -> Result<HashMap<String, String>> {
    let file = match File::open(file_path) {
        Ok(file) => {file},
        Err(_) => return Ok(HashMap::new()),
    }; // 打开 AOF 文件
    let reader = BufReader::new(file);
    let mut map = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let tokens: Vec<&str> = line.split_whitespace().collect();
        match tokens.get(0) {
            // 根据命令类型解析命令参数并更新 HashMap
            Some(&"SET") => {
                if tokens.len() == 3 {
                    map.insert(tokens[1].to_string(), tokens[2].to_string());
                }
            }
            // Some(&"DEL") => {
            //     if tokens.len() == 2 {
            //         map.remove(tokens[1]);
            //         println!("DEL: {}",tokens[1]);
            //     }
            // }
            _ => (),
        }
    }
    Ok(map)
}
