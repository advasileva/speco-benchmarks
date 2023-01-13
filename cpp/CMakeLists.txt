cmake_minimum_required(VERSION 3.21)
project(Benchmarking)

set(CMAKE_CXX_STANDARD 20)

add_executable(Benchmarking main.cpp Examples/Product.cpp Examples/Product.h Examples/Book.cpp Examples/Book.h Examples/Movie.cpp Examples/Movie.h Examples/Cart.cpp Examples/Cart.h Examples/Cart2.cpp Examples/Cart2.h Examples/Cart1.cpp Examples/Cart1.h Examples/Measurable.h Examples/Measurable.cpp)
