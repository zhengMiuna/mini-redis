use colored::*;
use std::process::{exit, Command};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // 启动服务端
    let mut server = Command::new("cargo")
        .args(&["run", "--bin", "server"])
        .spawn()
        .expect("Failed to start server");

    // 等待一段时间确保服务器已启动
    sleep(Duration::from_secs(2));

    // 客户端发三条 SET 操作
    let mut client_process = Command::new("cargo")
        .args(&["run", "--bin", "client", "set", "key1", "value1"])
        .spawn()
        .expect("Failed to start client");
    let status = client_process.wait().expect("Failed to wait for client");
    if !status.success() {
        // 客户端发送 Set 操作失败
        println!("Client failed to set key");
        exit(1);
    }
    
    sleep(Duration::from_secs(1));

    client_process = Command::new("cargo")
        .args(&["run", "--bin", "client", "set", "key2", "value2"])
        .spawn()
        .expect("Failed to start client");
    let status = client_process.wait().expect("Failed to wait for client");
    if !status.success() {
        // 客户端发送 Set 操作失败
        println!("Client failed to set key");
        exit(1);
    }

    sleep(Duration::from_secs(1));

    client_process = Command::new("cargo")
        .args(&["run", "--bin", "client", "set", "key3", "value3"])
        .spawn()
        .expect("Failed to start client");
    let status = client_process.wait().expect("Failed to wait for client");
    if !status.success() {
        // 客户端发送 Set 操作失败
        println!("Client failed to set key");
        exit(1);
    }

    // 停止服务端
    server.kill().expect("Failed to stop server");

    // 等待一段时间确保服务器已停止
    sleep(Duration::from_secs(2));

    println!("{}", "重启服务器".red());
    // 重新启动服务端
    let mut server = Command::new("cargo")
        .args(&["run", "--bin", "server"])
        .spawn()
        .expect("Failed to start server");

    // 等待一段时间确保服务器已启动
    sleep(Duration::from_secs(2));

    // 客户端发送三条 GET 操作
    client_process = Command::new("cargo")
        .args(&["run", "--bin", "client", "get", "key1"])
        .spawn()
        .expect("Failed to execute client");
    let output = client_process
        .wait_with_output()
        .expect("Failed to wait for client");
    if !output.status.success() {
        // 客户端发送 Get 操作失败
        println!("Client failed to get value");
        exit(1);
    }

    sleep(Duration::from_secs(1));

    client_process = Command::new("cargo")
        .args(&["run", "--bin", "client", "get", "key2"])
        .spawn()
        .expect("Failed to execute client");
    let output = client_process
        .wait_with_output()
        .expect("Failed to wait for client");
    if !output.status.success() {
        println!("Client failed to get value");
        exit(1);
    }

    sleep(Duration::from_secs(1));

    client_process = Command::new("cargo")
        .args(&["run", "--bin", "client", "get", "key3"])
        .spawn()
        .expect("Failed to execute client");
    let output = client_process
        .wait_with_output()
        .expect("Failed to wait for client");
    if !output.status.success() {
        println!("Client failed to get value");
        exit(1);
    }
    
    // 停止服务端
    server.kill().expect("Failed to stop server");
}
