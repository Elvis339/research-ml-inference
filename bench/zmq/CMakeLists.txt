cmake_minimum_required(VERSION 3.0 FATAL_ERROR)
project(zmq_bench)

find_package(cppzmq REQUIRED)

include_directories(${ZMQ_INCLUDE_DIRS})

add_executable(server server.cpp)
target_link_libraries(server cppzmq)

add_executable(client client.cpp)
target_link_libraries(client cppzmq)
