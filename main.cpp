#include <iostream>
#include <thread>
#include <memory>
#include <vector>

#include "any"
#include "zmq.hpp"
#include "zhelpers.hpp"
#include "src/AntiFraud.h"
#include "src/CliArgParser.h"
#include "schema_generated.h"
#include "BS_thread_pool.hpp"

uint64_t timeSinceEpochMillisec() {
    using namespace std::chrono;
    return duration_cast<milliseconds>(system_clock::now().time_since_epoch()).count();
}

std::vector<double> antiFraudInference(const uint8_t* buf, size_t size, AntiFraud* anti_fraud) {
    flatbuffers::Verifier verifier(buf, size);

    if (!VerifyAntiFraudInputBuffer(verifier)) {
        throw std::runtime_error("Buffer verification failed!");
    }
    const AntiFraudInput* af_input = GetAntiFraudInput(buf);
    auto fb_vector = af_input->inputs();

    std::vector<double> std_vector;
    if (fb_vector) {
        std_vector.reserve(fb_vector->size());
        for (double val : *fb_vector) {
            std_vector.push_back(val);
        }
    }

    auto tensor = anti_fraud->run(std_vector);
    return AntiFraud::to_vec(tensor);
}

zmq::message_t serializeResponse(const std::vector<double>& antiFraudInferenceResult) {
    flatbuffers::FlatBufferBuilder builder;
    auto responseVector = builder.CreateVector(antiFraudInferenceResult);
    AntiFraudResponseBuilder responseBuilder(builder);
    responseBuilder.add_response(responseVector);
    auto afResponse = responseBuilder.Finish();
    builder.Finish(afResponse);

    uint8_t *buf = builder.GetBufferPointer();
    size_t size = builder.GetSize();
    zmq::message_t message(buf, size);
    return message;
}

void workerTask(zmq::context_t& context, AntiFraud* anti_fraud, std::string& dealerAddress, BS::thread_pool& pool) {
    auto cerrCleanup = []() -> void { std::cerr.flush(); };

    try {
        // when socket goes out of scope it will be destroyed and closed
        zmq::socket_t socket(context, ZMQ_REP);
        // Set identity
        auto id = s_set_id(socket);
        socket.connect(dealerAddress);

        while (true) {
            zmq::message_t rx_msg;
            // In case of bad recv break the loop and terminate
            if (!socket.recv(rx_msg, zmq::recv_flags::none)) {
                std::cout << "[worker(" << timeSinceEpochMillisec() << ")]:" << "id=" << id << " bad recv shutting down\n";
                std::cout.flush();
                break;
            }

            // Extract the data pointer and size from the message
            const uint8_t* data = rx_msg.data<uint8_t>();
            size_t size = rx_msg.size();

            auto result = serializeResponse(antiFraudInference(data, size, anti_fraud));
            socket.send(result, zmq::send_flags::none);
            std::cout << "[worker(" << timeSinceEpochMillisec() << ")]:" << "id=" << id << "\n";
        }
    } catch (const zmq::error_t& e) {
        std::cerr << "ZMQ error in workerTask: " << e.what() << std::endl;
        cerrCleanup();
    }  catch (const std::exception& e) {
        std::cerr << "Standard exception in workerTask: " << e.what() << std::endl;
        cerrCleanup();
    } catch (...) {
        std::cerr << "Unknown exception in workerTask." << std::endl;
        cerrCleanup();
    }
}

void runBroker(AntiFraud* anti_fraud, int workers, std::string& routerAddress, std::string& dealerAddress) {
    zmq::context_t context(2);
    zmq::socket_t frontend(context, ZMQ_ROUTER); // Client-facing socket
    zmq::socket_t backend(context, ZMQ_DEALER);  // Worker-facing socket

    frontend.bind(routerAddress); // For clients
    backend.bind(dealerAddress); // For worker threads

    BS::thread_pool pool(workers);

    for (auto i = 0; i < workers; i++) {
        pool.push_task(workerTask, std::ref(context), anti_fraud, std::ref(dealerAddress), std::ref(pool));
    }


//    std::vector<std::thread> workerThreads;
//    workerThreads.reserve(workers);
//
//    for (int i = 0; i < workers; ++i) {
//        std::thread t(workerTask, std::ref(context), anti_fraud, std::ref(dealerAddress), std::ref(pool));
//        workerThreads.push_back(std::move(t));
//    }
//
//    for (auto& t : workerThreads) {
//        t.detach();
//    }

//    std::vector<std::thread> workerThreads;
//    workerThreads.reserve(workers);
//
//    // Start worker threads
//    for (int i = 0; i < workers; ++i) {
//        workerThreads.emplace_back(workerTask, std::ref(context), anti_fraud, std::ref(dealerAddress));
//    }

    std::cout << "[broker(" << timeSinceEpochMillisec() << ")]:" << "created thread pool of=" << workers << " workers\n";
    // Use ZeroMQ's proxy function to handle forwarding between sockets
    zmq::proxy(static_cast<void*>(frontend), static_cast<void*>(backend), nullptr);
}

int main(int argc, const char *argv[])
{
    std::string model_path;
    int workers;
    std::string routerAddress;
    std::string dealerAddress;

    CliArgParser cli = CliArgParser();
    cli.parse(argc, argv);

    try {
        auto modelPathCmd = std::any_cast<std::string>(cli.getCommandValue("model-path"));
        model_path = modelPathCmd;

        auto routerAddressCmd = std::any_cast<std::string>(cli.getCommandValue("router"));
        routerAddress = routerAddressCmd;

        auto dealerAddressCmd = std::any_cast<std::string>(cli.getCommandValue("dealer"));
        dealerAddress = dealerAddressCmd;

        auto workersCmd = std::any_cast<int>(cli.getCommandValue("workers"));
        workers = workersCmd;
    } catch (const std::bad_any_cast& e) {
        std::cerr << e.what() << std::endl;
    }

    if (workers <= 0) {
        workers = 10;
    }

    std::unique_ptr<AntiFraud> antiFraud = std::make_unique<AntiFraud>(model_path);
    runBroker(antiFraud.get(), workers, routerAddress, dealerAddress);
    std::cout << "Execution completed successfully.\n";
    return 0;
}