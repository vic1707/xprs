#!/bin/sh

## Because valgrind is not supported on arm macOS
## so we use docker to run valgrind on linux

docker image rm -f valgrind
docker build -t valgrind .
docker run -it --rm -v $(pwd):/home/valgrind valgrind