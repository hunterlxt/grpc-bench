
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

/************ Configuration ************/
const size_t CQ_NUM = 4;
const size_t MIN_POLLERS = 2;
const size_t MAX_POLLERS = 2;
/************ Configuration ************/

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
    builder.SetSyncServerOption(ServerBuilder::SyncServerOption::NUM_CQS,
                                CQ_NUM);
    builder.SetSyncServerOption(ServerBuilder::SyncServerOption::MIN_POLLERS,
                                MIN_POLLERS);
    builder.SetSyncServerOption(ServerBuilder::SyncServerOption::MAX_POLLERS,
                                MAX_POLLERS);
    std::unique_ptr<Server> server(builder.BuildAndStart());
    std::cout << "Server listening on " << server_address << std::endl;
    server->Wait();
}

int main(int argc, char **argv) {
    RunEchoServer();
    return 0;
}