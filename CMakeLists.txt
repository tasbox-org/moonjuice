cmake_minimum_required(VERSION 3.28)

project(MoonJuice CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

include(cmake/RegisterPublicPackage.cmake)

RegisterPublicPackage(moonjuice-common)
RegisterPublicPackage(moonjuice-lexer)
RegisterPublicPackage(moonjuice-parser)
RegisterPublicPackage(moonjuice-transpiler-luau)

#if (NOT MOONJUICE_DISABLE_TESTS)
#  add_subdirectory(tests)
#endif()
