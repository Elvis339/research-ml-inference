//
// Created by Elvis Sabanovic on 12.12.23..
//

#include "CliArgParser.h"

CliArgParser::CliArgParser() {
    commands_["help"] = Command {
        false,
        "Print commands to the console",
    };

    commands_["model-path"] = Command {
            true,
            "Path to .pt file",
    };


    commands_["workers"] = Command {
        false,
        "Set max number of worker threads",
        10
    };

    commands_["router"] = Command {
        false,
        "Set router address",
        std::string("ipc:///tmp/router"),
    };

    commands_["dealer"] = Command {
            false,
            "Set router address",
            std::string("ipc:///tmp/worker")
    };
}


void CliArgParser::parse(int argc, const char *argv[]) {
    std::map<std::string, bool> required;

    for (auto const& [key, val] : commands_) {
        if (val.required) {
            required.insert(std::make_pair(key, true));
        }
    }

    for (auto i = 1; i < argc; i++) {
        std::string command;
        auto cmd = argv[i];
        command = cmd;

        // todo: fix out of bonds arr access
        auto value = argv[i + 1];

        if (commands_.find(command) != commands_.end()) {
            if (command == "workers") {
                auto workers = atoi(reinterpret_cast<const char *>(value));
                updateCommand("workers", int(workers));
            } else {
                if (command != "help") {
                    updateCommand(command, std::string(value));
                }
            }
        }

        // If command exists remove it from the required map
        required.erase(command);
    }

    if (!required.empty()) {
        help();
        throw std::runtime_error("Missing required args!");
    }

    std::cout << "=== Commands set: ===\n";
    for (const auto& [key, val] : commands_) {
        if (val.value.has_value()) {
            std::cout << "    Command: "     << key << "\n";
            std::cout << "    Description: " << val.description << "\n";
            if (val.value.type() == typeid(std::string)) {
                std::cout << "    Value: " << std::any_cast<std::string>(val.value) << "\n";
            } else if (val.value.type() == typeid(int)) {
                std::cout << "    Value: " << std::any_cast<int>(val.value) << "\n";
            } else {
                std::cout << "    Value: " << "COULD NOT CAST!" << "\n";
            }
            std::cout << "\n";
        }
    }
    std::cout << "==========\n";
}

std::any CliArgParser::getCommandValue(const std::string &commandName) {
    auto it = commands_.find(commandName);

    if (it != commands_.end()) {
        if (it->second.value.has_value()) {
            return it->second.value;
        }
        return it->second.defaultValue;
    }

    throw std::runtime_error("Command not found.");
}

void CliArgParser::updateCommand(const std::string &commandName, const std::any &value) {
    auto it = commands_.find(commandName);
    if (it != commands_.end()) {
        it->second.value = value;
    }
}

void CliArgParser::help() {
    std::cout << "Available Commands:\n";
    for (const auto& [key, val] : commands_) {

        std::cout << "    Command: " << key << "\n";
        std::cout << "    Required: " << (val.required ? "Yes" : "No") << "\n";
        std::cout << "    Description: " << val.description << "\n";

        if (val.defaultValue.has_value()) {
            if (val.defaultValue.type() == typeid(std::string)) {
                std::cout << "    Default Value: " << std::any_cast<std::string>(val.defaultValue) << "\n";
            } else if (val.defaultValue.type() == typeid(int)) {
                std::cout << "    Default Value: " << std::any_cast<int>(val.defaultValue) << "\n";
            } else {
                std::cout << "    Default Value: " << "COULD NOT CAST!" << "\n";
            }
        }
        std::cout << "\n";
    }
}


CliArgParser::~CliArgParser() = default;
