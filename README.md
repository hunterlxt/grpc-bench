# grpc-bench
some bench case for grpc-rs

## Rust Usage

Run the test case which you want.

```bash
cargo run --release --bin server -- --case test --cq 4 --port 50000 --quota_size 1000000
cargo run --release --bin client -- --case send_stream --port 50000 --cq 2 --thread_num 1 --msg_num 1 --msg_size 1000 --quota_size 1000000
```

## C++ Usage

`cpp-bench` is to compare performance of the Rust. Because Rust and CPP use the same proto protocol, you can even use Rust as server and CPP as client.

You need install grpc first

```bash
make
./server 1 1 1
./client 4 1000 1000
make clean
```
