//
// Created by Elvis Sabanovic on 2.12.23.
//

#include "AntiFraud.h"
#include <regex>

AntiFraud::AntiFraud(const std::string& path) {
    anti_fraud = load(path);
    if (!anti_fraud) {
        throw std::runtime_error("Failed to load the model");
    }
    std::cout << "[anti_fraud]: model loaded\n";
}

std::unique_ptr<torch::jit::script::Module> AntiFraud::load(const std::string& path) {
    try {
        return std::make_unique<torch::jit::script::Module>(torch::jit::load(path));
    } catch (const c10::Error& e) {
        std::cerr << "Error loading the model: " << e.what() << std::endl;
        return nullptr;
    }
}

at::Tensor AntiFraud::run(std::vector<double> input_data) const {
    auto opts = torch::TensorOptions().dtype(torch::kDouble);
    torch::Tensor input_tensor = torch::from_blob(input_data.data(), {(int)input_data.size(), 1}, opts);
    std::vector<torch::jit::IValue> inputs;
    inputs.push_back(input_tensor);
    at::Tensor output_tensor = anti_fraud->forward(inputs).toTensor();
    return output_tensor.data();
}

std::vector<double> AntiFraud::to_vec(const at::Tensor &tensor) {
    auto t = tensor.contiguous();
    std::vector<double> v(t.data_ptr<double>(), t.data_ptr<double>() + t.numel());
    return v;
}

AntiFraud::~AntiFraud() = default;
