set(PROJ dataStructures)
project(${PROJ})

add_library(${PROJ})

target_sources(${PROJ} 
    PUBLIC 
    src/Vector.cpp)

target_include_directories(${PROJ}
    PUBLIC
    include)

add_subdirectory(test)
