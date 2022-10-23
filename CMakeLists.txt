cmake_minimum_required(VERSION 3.15.1)

project(SwiftSimple LANGUAGES C Swift)
set(CMAKE_Swift_FLAGS "${CMAKE_Swift_FLAGS} -import-objc-header BridgingHeader.h")

add_subdirectory(Externals)
add_executable(HelloWorld
  main.swift
  BridgingHeader.h)

target_include_directories(HelloWorld PRIVATE Externals)
target_link_directories(HelloWorld PRIVATE Externals/rust/target)
target_link_libraries(HelloWorld PRIVATE MyLib my_rust_lib)