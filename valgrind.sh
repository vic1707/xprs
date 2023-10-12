#!/bin/sh

docker image rm -f valgrind
docker build -t valgrind .
docker run -it --rm -v $(pwd):/home/valgrind valgrind sh -c "\
    cd /home/valgrind && \
    cargo build --release && \
    valgrind \
        --leak-check=full \
        --show-leak-kinds=all \
        --track-origins=yes \
        --verbose \
        --log-file=valgrind-out.txt \
        ./target/release/repl"
