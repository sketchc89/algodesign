set(PROJ testCodingProblems)
project(${PROJ})

add_executable(${PROJ})

target_sources(${PROJ}
    PUBLIC
    src/main.cpp
    src/TestProblems.cpp)

target_include_directories(${PROJ}
    PUBLIC
    include
    "${gtest_SOURCE_DIR}/include"
    "${gmock_SOURCE_DIR}/include")

target_link_libraries(${PROJ}
    gmock
    gmock_main
    codingProblems)
