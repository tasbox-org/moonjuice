cmake_minimum_required(VERSION 3.28)

find_package(Catch2 CONFIG REQUIRED)

include(Catch)

# Traditionally, you shouldn't GLOB source files as it breaks changed file detection.
# However, the "new" CONFIGURE_DEPENDS parameter solves this for most cases with minimal build-time overhead
file(GLOB_RECURSE SOURCES CONFIGURE_DEPENDS "packages/*.cpp" "tests/*.cpp")

add_executable(MoonJuice_Tests ${SOURCES})

target_link_libraries(
        MoonJuice_Tests PRIVATE
        Catch2::Catch2WithMain
)

target_include_directories(
        MoonJuice_Tests PRIVATE
        packages
        tests
)
