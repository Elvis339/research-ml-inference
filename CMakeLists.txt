cmake_minimum_required(VERSION 3.0 FATAL_ERROR)
project(anti_fraud)

if (CMAKE_SYSTEM_NAME STREQUAL "Linux")
    set(CMAKE_PREFIX_PATH "/home/ec2-user/libtorch")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -pthread")
elseif (CMAKE_SYSTEM_NAME STREQUAL "Darwin")
    set(CMAKE_PREFIX_PATH "/opt/homebrew/Cellar/pytorch/2.1.0_1;/opt/homebrew/Cellar/zeromq/4.3.5_1")
endif()

find_package(Torch REQUIRED)
find_package(cppzmq REQUIRED)
find_package(nlohmann_json 3.2.0 REQUIRED)

include_directories(${ZMQ_INCLUDE_DIRS})
file(GLOB SOURCES "src/*.cpp" "src/*.h")

add_executable(anti_fraud main.cpp
        AntiFraud.cpp
        AntiFraud.h
        schema_generated.h
        zhelpers.hpp
        ${SOURCES}
)
target_link_libraries(anti_fraud "${TORCH_LIBRARIES}")
target_link_libraries(anti_fraud cppzmq)
set_property(TARGET anti_fraud PROPERTY CXX_STANDARD 17)