#include "proto/test.grpc.pb.h"
#include "util.h"
#include <ctime>
#include <grpcpp/grpcpp.h>
#include <iostream>
#include <memory>
#include <string>

using grpc::Channel;
using grpc::ClientContext;
using grpc::Status;
using test::RpcRequest;
using test::RpcResponse;
using test::TestService;

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

int main(int argc, char **argv) {
    RpcClient client(grpc::CreateChannel("localhost:50051",
                                         grpc::InsecureChannelCredentials()));
    std::string data = generate_string(1024 * 1024);
    clock_t start = clock();
    for (int i = 0; i < 100000; i++) {
        client.unary(data);
    }
    clock_t finish = clock();
    double consumeTime = (double)(finish - start) / CLOCKS_PER_SEC;
    std::cout << "Time usage:" << consumeTime << std::endl;
    return 0;
}