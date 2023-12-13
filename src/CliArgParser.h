//
// Created by Elvis Sabanovic on 12.12.23..
//

#ifndef ANTI_FRAUD_CLIARGPARSER_H
#define ANTI_FRAUD_CLIARGPARSER_H

#include <any>
#include <iostream>
#include <sstream>
#include <variant>
#include <string>
#include <map>
#include <vector>

struct Command {
    bool required;
    std::string description;
    std::any defaultValue;
    std::any value;
};

class CliArgParser {
public:
    CliArgParser();
    ~CliArgParser();
    void parse(int argc, const char *argv[]);
    void help();
    void updateCommand(const std::string& commandName, const std::any& value);
    std::any getCommandValue(const std::string& commandName);
private:
    std::map<std::string, Command> commands_;
};


#endif //ANTI_FRAUD_CLIARGPARSER_H
