//
// Created by Elvis Sabanovic on 2.12.23..
//

#ifndef ANTI_FRAUD_ANTIFRAUD_H
#define ANTI_FRAUD_ANTIFRAUD_H


#include <iostream>
#include <memory>
#include <string>
#include <vector>
#include <torch/script.h>

class AntiFraud {
public:
    std::unique_ptr<torch::jit::script::Module> anti_fraud;
    explicit AntiFraud(const std::string& path);
    at::Tensor run(std::vector<float> data) const;
    static std::vector<double> to_vec(const at::Tensor& tensor);
    ~AntiFraud();
private:
    std::unique_ptr<torch::jit::script::Module> load(const std::string& path);
};


#endif //ANTI_FRAUD_ANTIFRAUD_H
