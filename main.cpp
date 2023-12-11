#include <future>
#include <iostream>
#include <thread>
#include <memory>
#include <vector>
#include <chrono>

#include "zmq.hpp"
#include "AntiFraud.h"

#include <nlohmann/json.hpp>
using json = nlohmann::json;

const std::string SUBSCRIBER_ADDR = "ipc:///tmp/zeromq_sub_uds";
const std::string MODEL_RESULT_PUBLISHER_ADDR = "ipc:///tmp/zeromq_pub_uds";

void ProcessAndPublish(zmq::socket_t* socket, AntiFraud* antiFraud, zmq::message_t message) {
    json parsed_data = json::parse(message.to_string());
    std::vector<double> weights = parsed_data["model_inputs"].get<std::vector<double>>();

    auto start_model_execution_metric_nano = std::chrono::high_resolution_clock::now();
    auto af_response = AntiFraud::to_vec(antiFraud->run(std::move(weights)));
    auto finish_model_execution_metric_nano = std::chrono::high_resolution_clock::now();

    parsed_data["result"] = af_response[0];
    parsed_data["inference_time_ns"] = std::chrono::duration_cast<std::chrono::nanoseconds>(finish_model_execution_metric_nano-start_model_execution_metric_nano).count();

    auto meta = to_string(parsed_data);
    std::cout << "[anti-fraud(inference_time_ns)]: " << parsed_data["inference_time_ns"] << "\n";

    zmq::message_t msg(meta.size());
    memcpy(msg.data(), meta.data(), meta.size());

    try {
        socket->send(msg, zmq::send_flags::dontwait);
        msg.rebuild(meta.size());
    } catch(zmq::error_t &e) {
        std::cout << e.what() << std::endl;
    }
}

void Processor(zmq::context_t* ctx, AntiFraud* antiFraud)
{
    zmq::socket_t publisher_socket(*ctx, zmq::socket_type::pub);
    publisher_socket.bind(MODEL_RESULT_PUBLISHER_ADDR);
    std::this_thread::sleep_for(std::chrono::milliseconds (1000));


    zmq::socket_t subscriber(*ctx, zmq::socket_type::sub);
    subscriber.connect(SUBSCRIBER_ADDR);
    subscriber.set(zmq::sockopt::subscribe, "");

    std::cout << "[processor]: started\n";
    while (true) {
        zmq::message_t rx_msg;
        subscriber.recv(rx_msg, zmq::recv_flags::none);

//        std::string msg = std::string(static_cast<char*>(rx_msg.data()), rx_msg.size());


        ProcessAndPublish(&publisher_socket, antiFraud, std::move(rx_msg));
//
//        std::thread processingThread(ProcessAndPublish, &publisher_socket, antiFraud, std::move(rx_msg));
//        processingThread.join();
    }
}

int main(int argc, const char *argv[])
{
    auto model_path = "/Users/elvissabanovic/Projects/research-ml-inference/data/anti_fraud_model.pt";
    std::unique_ptr<AntiFraud> antiFraud = std::make_unique<AntiFraud>(model_path);
    zmq::context_t ctx(1);
    Processor(&ctx, antiFraud.get());
    std::cout << "Execution completed successfully.\n";
    return 0;
}