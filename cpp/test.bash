#!/bin/bash

DIR=$(dirname $0)
PROG=${DIR}/build/dataStructures/test/testDataStructures

# ${PROG} & PID_PROG=$!
# echo ${PID_PROG}
# sleep 2

if hash valgrind; then
    valgrind --tool=memcheck --leak-check=full ${PROG}
fi
