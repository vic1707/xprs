FROM alpine:latest
RUN apk add valgrind cargo
RUN cargo install cargo-valgrind

WORKDIR /home/valgrind

# CMD [ "cargo", "valgrind", "run"]
CMD [ "cargo", "valgrind", "test"]
