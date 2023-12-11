#include <zmq.hpp>
#include <iostream>
#include <chrono>
#include <vector>
#include <random>
#include <ctime>
#include <fstream>

std::vector<float> random_model_inputs(size_t length) {
    std::vector<float> randomFloats;
    randomFloats.reserve(length);

    // Initialize random number generator
    std::mt19937 generator(static_cast<unsigned int>(std::time(nullptr))); // Seed with current time
    std::uniform_real_distribution<float> distribution(0.0f, 2.0f); // Set range

    for (size_t i = 0; i < length; ++i) {
        randomFloats.push_back(distribution(generator));
    }

    return randomFloats;
}

int main(int argc, char* argv[]) {
    std::string connection = argv[1];
    std::string filename;

    if (connection == "uds") {
        connection = "ipc:///tmp/pingpong";
        filename = "uds_metrics.csv";
    } else {
        connection = "tcp://localhost:5555";
        filename = "tcp_metrics.csv";
    }

    std::cout << "[client]: connected to" << connection << "\n";

    std::ofstream file(filename, std::ios::app);
    file << "id,response_time_ms,size_bytes\n";

    zmq::context_t context(1);
    zmq::socket_t socket(context, 3);
    socket.connect(connection);


    auto mock_model_data = random_model_inputs(41);

    const int numRequests = 1000000;
    for (int i = 0; i < numRequests; ++i) {
        auto start = std::chrono::high_resolution_clock::now();

        zmq::message_t request(mock_model_data.size());
        memcpy(request.data(), mock_model_data.data(), mock_model_data.size());
        socket.send(request, 0);

        zmq::message_t reply;
        socket.recv(&reply);
        auto end = std::chrono::high_resolution_clock::now();

        std::chrono::duration<double, std::milli> elapsed = end - start;

        std::cout << "Request #" << (i + 1)
                  << ": Response time = " << elapsed.count() << " ms\n";

        file << (i + 1) << ",";
        file << elapsed.count() << ",";
        file << mock_model_data.size() << "\n";
    }

    file.close();

    return 0;
}
