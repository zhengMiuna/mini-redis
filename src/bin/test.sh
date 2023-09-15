#!/bin/bash

# 启动服务端
cargo run --bin server &

# 等待一段时间确保服务器已启动
sleep 1

# 客户端发三条 SET 操作
cargo run --bin client set key1 value1
if [ $? -ne 0 ]; then
    # 客户端发送 Set 操作失败
    echo "Client failed to set key"
    exit 1
fi

sleep 1

cargo run --bin client set key2 value2
if [ $? -ne 0 ]; then
    # 客户端发送 Set 操作失败
    echo "Client failed to set key"
    exit 1
fi

sleep 1

cargo run --bin client set key3 value3
if [ $? -ne 0 ]; then
    # 客户端发送 Set 操作失败
    echo "Client failed to set key"
    exit 1
fi

# 停止服务端
killall server

# 等待一段时间确保服务器已停止
sleep 1

echo -e "\033[31m重启服务器\033[0m"
# 重新启动服务端
cargo run --bin server &

# 等待一段时间确保服务器已启动
sleep 1

# 客户端发送三条 GET 操作
cargo run --bin client get key1
output=$(cargo run --bin client get key1)
if [ $? -ne 0 ]; then
    # 客户端发送 Get 操作失败
    echo "Client failed to get value"
    exit 1
fi

sleep 1

cargo run --bin client get key2
output=$(cargo run --bin client get key2)
if [ $? -ne 0 ]; then
    echo "Client failed to get value"
    exit 1
fi

sleep 1

cargo run --bin client get key3
output=$(cargo run --bin client get key3)
if [ $? -ne 0 ]; then
    echo "Client failed to get value"
    exit 1
fi

# 停止服务端
killall server
