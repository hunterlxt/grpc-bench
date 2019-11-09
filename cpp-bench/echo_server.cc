
#include "proto/test.grpc.pb.h"
#include "util.h"
#include <chrono>
#include <grpcpp/grpcpp.h>
#include <iostream>
#include <memory>
#include <string>
#include <thread>

using grpc::Server;
using grpc::ServerBuilder;
using grpc::ServerContext;
using grpc::ServerReader;
using grpc::Status;
using test::RpcRequest;
using test::RpcResponse;
using test::TestService;

/************ Default Configuration ************/
size_t CQ_NUM = 1;
size_t MIN_POLLERS = 1;
size_t MAX_POLLERS = 1;
bool NEED_CALC = false;
size_t CALC_TIMES = 1844674407; // about 3 minutes
bool NEED_SLEEP = false;
size_t SLEEP_TIME = 1;
/************ Default Configuration ************/

void calc_to_waste_time() {
    for (size_t i = 0; i < CALC_TIMES; i++)
        ;
}

class TestServiceImpl final : public TestService::Service {
    // return original message to client
    Status GetUnary(ServerContext *context, const RpcRequest *request,
                    RpcResponse *reply) override {
        if (NEED_CALC)
            calc_to_waste_time();
        if (NEED_SLEEP)
            std::this_thread::sleep_for(std::chrono::seconds(1));
        reply->set_data(request->data());
        return Status::OK;
    }

    // return num of messages from client
    Status SendStream(ServerContext *context, ServerReader<RpcRequest> *reader,
                      RpcResponse *reply) override {
        RpcRequest req;
        int count = 0;
        while (reader->Read(&req)) {
            if (NEED_CALC)
                calc_to_waste_time();
            if (NEED_SLEEP)
                std::this_thread::sleep_for(std::chrono::seconds(1));
            count++;
        }
        reply->set_data(generate_string(count));
        return Status::OK;
    }
};

void RunServer() {
    std::string server_address("0.0.0.0:50051");
    TestServiceImpl service;
    ServerBuilder builder;
    builder.AddListeningPort(server_address, grpc::InsecureServerCredentials());
    builder.RegisterService(&service);
    builder.SetDefaultCompressionAlgorithm(GRPC_COMPRESS_NONE);
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
    for (size_t i = 1; i < argc; i += 1) {
        switch (i) {
        case 1:
            CQ_NUM = atoi(argv[i]);
            break;
        case 2:
            MIN_POLLERS = atoi(argv[i]);
            break;
        case 3:
            MAX_POLLERS = atoi(argv[i]);
            break;
        }
    }
    RunServer();
    return 0;
}