set(PROJ codingProblems)
project(${PROJ})

add_library(${PROJ} 
    src/SingleElementInASortedArray.cpp)

target_include_directories(${PROJ}
    PUBLIC
    include)

add_subdirectory(test)
