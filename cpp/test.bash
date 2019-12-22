#!/bin/bash

DIR=$(dirname $0)
echo ${DIR}
PROG=${DIR}/build/dataStructures/test/testDataStructures

${PROG}

if hash valgrind; then 
    valgrind --tool=memcheck --leak-check=full --show-leak-kinds=all ${PROG}
fi
