#include <iostream>
#include <thread>
#include <memory>

#include "zmq.hpp"

int main(int argc, char* argv[]) {
    std::string connection = argv[1];

    if (connection == "uds") {
        connection = "ipc:///tmp/pingpong";
    } else {
        connection = "tcp://localhost:5555";
    }

    zmq::context_t context(1);
    zmq::socket_t socket(context, 4);
    socket.bind(connection);

    std::cout << "Server is running on " << connection << std::endl;

    while (true) {
        zmq::message_t request;
        socket.recv(&request, 0);

        zmq::message_t reply(4);
        memcpy(reply.data(), "PONG", 4);
        socket.send(reply, 0);
        std::cout << "[server]: replied\n";
    }
    return 0;
}
