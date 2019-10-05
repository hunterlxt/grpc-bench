
#include "proto/test.grpc.pb.h"
#include <grpcpp/grpcpp.h>
#include <iostream>
#include <memory>
#include <string>

using grpc::Server;
using grpc::ServerBuilder;
using grpc::ServerContext;
using grpc::ServerReader;
using grpc::Status;
using test::RpcRequest;
using test::RpcResponse;
using test::TestService;

// Logic and data behind the server's behavior.
class TestServiceImpl final : public TestService::Service {
    Status GetUnary(ServerContext *context, const RpcRequest *request,
                    RpcResponse *reply) override {
        reply->set_data(request->data());
        return Status::OK;
    }
};

void RunEchoServer() {
    std::string server_address("0.0.0.0:50051");
    TestServiceImpl service;
    ServerBuilder builder;
    builder.AddListeningPort(server_address, grpc::InsecureServerCredentials());
    builder.RegisterService(&service);
    std::unique_ptr<Server> server(builder.BuildAndStart());
    std::cout << "Server listening on " << server_address << std::endl;
    server->Wait();
}

int main(int argc, char **argv) {
    RunEchoServer();
    return 0;
}