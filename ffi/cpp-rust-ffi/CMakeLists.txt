cmake_minimum_required(VERSION 3.0)
project(example)

add_executable(example cpp/main.cpp)

# Enable ExternalProject CMake module
include(ExternalProject)

# Set default ExternalProject root directory
set_directory_properties(PROPERTIES EP_PREFIX ${CMAKE_BINARY_DIR}/Rust)

# Add rust_example
ExternalProject_Add(
    rust_example
    DOWNLOAD_COMMAND ""
    CONFIGURE_COMMAND ""
    BUILD_COMMAND cargo build COMMAND cargo build --release
    BINARY_DIR "${CMAKE_SOURCE_DIR}/common-rust"
    INSTALL_COMMAND ""
    LOG_BUILD ON)

# Create dependency of Example on rust_example
add_dependencies(example rust_example)

# Specify Example's link libraries
target_link_libraries(example
    debug "${CMAKE_SOURCE_DIR}/common-rust/target/debug/librust_example.a"
    optimized "${CMAKE_SOURCE_DIR}/common-rust/target/release/librust_example.a")

set_target_properties(example PROPERTIES CXX_STANDARD 11 CXX_STANDARD_REQUIRED ON)
