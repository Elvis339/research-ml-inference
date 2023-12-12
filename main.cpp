#include <future>
#include <iostream>
#include <thread>
#include <memory>
#include <vector>
#include <stdio.h>

#include "zmq.hpp"
#include "AntiFraud.h"
#include "schema_generated.h"

std::vector<double> anti_fraud_inference(const uint8_t* buf, size_t size, AntiFraud* anti_fraud) {
    flatbuffers::Verifier verifier(buf, size);

    if (!VerifyAntiFraudInputBuffer(verifier)) {
        throw std::runtime_error("Buffer verification failed!");
    }
    const AntiFraudInput* af_input = GetAntiFraudInput(buf);
    auto fb_vector = af_input->inputs();

    std::vector<float> std_vector;
    if (fb_vector) {
        std_vector.reserve(fb_vector->size());
        for (float val : *fb_vector) {
            std_vector.push_back(val);
        }
    }

    auto tensor = anti_fraud->run(std_vector);
    return AntiFraud::to_vec(tensor);
}

zmq::message_t serialize_response(double anti_fraud_inference_result) {
    flatbuffers::FlatBufferBuilder builder;
    auto af_response = CreateAntiFraudResponse(builder, anti_fraud_inference_result);
    builder.Finish(af_response);
    uint8_t *buf = builder.GetBufferPointer();
    size_t size = builder.GetSize();
    zmq::message_t message(buf, size);
    return message;
}

void worker_task(zmq::context_t& context, int id, AntiFraud* anti_fraud) {
    zmq::socket_t socket(context, ZMQ_REP);
    socket.connect("ipc:///tmp/workers");

    while (true) {
        zmq::message_t rx_msg;
        socket.recv(rx_msg, zmq::recv_flags::none);

        // Extract the data pointer and size from the message
        const uint8_t* data = rx_msg.data<uint8_t>();
        size_t size = rx_msg.size();

        auto result = serialize_response(anti_fraud_inference(data, size, anti_fraud)[0]);
        socket.send(result);
        std::cout << "[worker(" << id << ")]: done\n";
    }
}

void run_broker(AntiFraud* anti_fraud, int number_of_workers) {
    zmq::context_t context(1);
    zmq::socket_t frontend(context, ZMQ_ROUTER); // Client-facing socket
    zmq::socket_t backend(context, ZMQ_DEALER);  // Worker-facing socket

    frontend.bind("ipc:///tmp/router"); // For clients
    backend.bind("ipc:///tmp/workers"); // For worker threads

    const int num_workers = number_of_workers; // Number of worker threads in the pool
    std::vector<std::thread> workers;

    // Start worker threads
    for (int i = 0; i < num_workers; ++i) {
        workers.emplace_back(worker_task, std::ref(context), i, anti_fraud);
    }

    std::cout << "[broker]: created thread pool of=" << num_workers << " workers\n";

    // Use ZeroMQ's proxy function to handle forwarding between sockets
    zmq::proxy(static_cast<void*>(frontend), static_cast<void*>(backend), nullptr);
}

int main(int argc, const char *argv[])
{
    const char* num_of_workers_str = argv[1];
    int num_of_workers = 0;
    num_of_workers = atoi(num_of_workers_str);

    if (num_of_workers <= 0) {
        num_of_workers = 10;
    }

    auto model_path = "/Users/elvissabanovic/Projects/research-ml-inference/data/anti_fraud_model.pt";
    std::unique_ptr<AntiFraud> antiFraud = std::make_unique<AntiFraud>(model_path);

    run_broker(antiFraud.get(), num_of_workers);
    std::cout << "Execution completed successfully.\n";
    return 0;
}