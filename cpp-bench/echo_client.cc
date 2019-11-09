#include "proto/test.grpc.pb.h"
#include "util.h"
#include <chrono>
#include <grpcpp/grpcpp.h>
#include <iostream>
#include <memory>
#include <string>
#include <thread>
#include <vector>

using grpc::Channel;
using grpc::ClientContext;
using grpc::Status;
using test::RpcRequest;
using test::RpcResponse;
using test::TestService;

typedef enum {
    UNARY,
    STREAM,
} Mode;

/************ Default Configuration ************/
size_t THREAD_NUM = 4;
size_t MSG_SIZE = 1000;
size_t LOOP_NUM = 1000;
Mode DEFAULT_MODE = UNARY;
/************ Default Configuration ************/

void loop_unary(std::string &data) {

    auto stub = TestService::NewStub(grpc::CreateChannel(
        "localhost:50051", grpc::InsecureChannelCredentials()));

    for (size_t i = 0; i < LOOP_NUM; i++) {
        RpcRequest request;
        request.set_data(data);
        RpcResponse reply;
        ClientContext context;
        Status status = stub->GetUnary(&context, request, &reply);
        if (status.ok()) {
            return;
        } else {
            std::cout << status.error_code() << ": " << status.error_message()
                      << std::endl;
            exit(-1);
        }
    }
}

void loop_stream(std::string &data) {
    auto stub = TestService::NewStub(grpc::CreateChannel(
        "localhost:50051", grpc::InsecureChannelCredentials()));

    RpcResponse reply;
    ClientContext context;
    auto stream = stub->SendStream(&context, &reply);

    for (size_t i = 0; i < LOOP_NUM; i++) {
        RpcRequest request;
        request.set_data(data);
        stream->Write(request);
    }
    stream->WritesDone();
    auto status = stream->Finish();
    if (status.ok()) {
        if (reply.data().length() != LOOP_NUM) {
            exit(-1);
        }
        return;
    } else {
        std::cout << status.error_code() << ": " << status.error_message()
                  << std::endl;
        exit(-1);
    }
}

void RunClient() {
    std::string data = generate_string(MSG_SIZE);
    auto start = std::chrono::system_clock::now();
    std::thread threads[THREAD_NUM];

    for (int i = 0; i < THREAD_NUM; i++) {
        switch (DEFAULT_MODE) {
        case UNARY:
            threads[i] = std::thread(loop_unary, std::ref(data));
            break;

        case STREAM:
            threads[i] = std::thread(loop_stream, std::ref(data));
            break;
        }
    }

    for (int i = 0; i < THREAD_NUM; i++) {
        threads[i].join();
    }

    auto finish = std::chrono::system_clock::now();
    auto consumeTime =
        std::chrono::duration_cast<std::chrono::microseconds>(finish - start)
            .count();
    std::cout << "Time usage:" << consumeTime * 1e-6 << std::endl;
}

int main(int argc, char **argv) {
    for (size_t i = 1; i < argc; i += 1) {
        switch (i) {
        case 1:
            THREAD_NUM = atoi(argv[i]);
            break;
        case 2:
            MSG_SIZE = atoi(argv[i]);
            break;
        case 3:
            LOOP_NUM = atoi(argv[i]);
            break;
        }
    }
    RunClient();
    return 0;
}
