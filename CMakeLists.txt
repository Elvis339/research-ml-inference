cmake_minimum_required(VERSION 3.0 FATAL_ERROR)
project(anti_fraud)

set(CMAKE_PREFIX_PATH "/opt/homebrew/Cellar/pytorch/2.1.0_1;/opt/homebrew/Cellar/zeromq/4.3.5_1")

find_package(Torch REQUIRED)
find_package(cppzmq REQUIRED)
find_package(nlohmann_json 3.2.0 REQUIRED)

include_directories(${ZMQ_INCLUDE_DIRS})

add_executable(anti_fraud main.cpp
        AntiFraud.cpp
        AntiFraud.h
)
target_link_libraries(anti_fraud "${TORCH_LIBRARIES}")
target_link_libraries(anti_fraud cppzmq)
target_link_libraries(anti_fraud nlohmann_json::nlohmann_json)
set_property(TARGET anti_fraud PROPERTY CXX_STANDARD 17)