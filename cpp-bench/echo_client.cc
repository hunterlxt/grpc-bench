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

/************ Configuration ************/
const size_t THREAD_NUM = 4;
const size_t LOOP_NUM = 1000000;
/************ Configuration ************/

class RpcClient {
  public:
    RpcClient(std::shared_ptr<Channel> channel)
        : stub_(TestService::NewStub(channel)) {}

    std::string unary(const std::string &user) {
        RpcRequest request;
        request.set_data(user);
        RpcResponse reply;
        ClientContext context;
        // The actual RPC.
        Status status = stub_->GetUnary(&context, request, &reply);
        if (status.ok()) {
            return reply.data();
        } else {
            std::cout << status.error_code() << ": " << status.error_message()
                      << std::endl;
            exit(-1);
        }
    }

  private:
    std::unique_ptr<TestService::Stub> stub_;
};

void loop_unary(std::string &data) {
    RpcClient client(grpc::CreateChannel("localhost:50051",
                                         grpc::InsecureChannelCredentials()));
    for (size_t i = 0; i < LOOP_NUM; i++) {
        client.unary(data);
    }
}

void run_echo_client() {
    std::string data = generate_string(64);
    auto start = std::chrono::system_clock::now();
    std::thread threads[THREAD_NUM];

    for (int i = 0; i < THREAD_NUM; i++) {
        threads[i] = std::thread(loop_unary, std::ref(data));
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
    run_echo_client();
    return 0;
}