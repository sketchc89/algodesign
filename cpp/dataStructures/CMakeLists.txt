set(PROJ dataStructures)
project(${PROJ})

add_library(${PROJ} INTERFACE)

target_include_directories(${PROJ}
    INTERFACE
    include)

add_subdirectory(test)
