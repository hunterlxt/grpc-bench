# grpc-bench
some bench case for grpc-rs

## Rust bench

Run the test case which you want.

```bash
cargo run --release --bin server -- --case ping_pong_2cq
cargo run --release --bin client -- --case ping_pong_1MB_10000
```

## C++ bench

`cpp-bench` is to compare performance of the Rust

```bash
# clean the workspace
make clean
# build the client & server to echo
make echo
```