CXX = g++
CPPFLAGS += `pkg-config --cflags protobuf grpc`
CXXFLAGS += -std=c++11
LDFLAGS += -L/usr/local/lib `pkg-config --libs protobuf grpc++ grpc`\
           -pthread\
           -Wl,--no-as-needed -lgrpc++_reflection -Wl,--as-needed\
           -ldl

clean:
	rm -rf *.o proto/*.o client server

echo: client server
client: proto/test.pb.o proto/test.grpc.pb.o echo_client.o
	$(CXX) $^ $(LDFLAGS) -o $@
server: proto/test.pb.o proto/test.grpc.pb.o echo_server.o
	$(CXX) $^ $(LDFLAGS) -o $@
