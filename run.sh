#!/bin/bash
cd ~/Documents/CODE/rusthw/mini-redis//target/debug/
./client set key1 value1
./client1 get key1
./client set key2 value2
./client1 set key3 value3