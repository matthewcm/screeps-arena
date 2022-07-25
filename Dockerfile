FROM rust:latest

RUN git clone https://github.com/matthewcm/cargo-screeps.git
WORKDIR /cargo-screeps
RUN cargo install --path .
RUN cargo install cargo-watch

WORKDIR /
RUN apt-get update && apt-get install -y wget build-essential pkg-config libssl-dev binaryen
RUN wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add -
RUN apt-get update
RUN apt-get install -y clang

VOLUME [ "/code", "screepsarena" ]

WORKDIR /code
CMD cargo watch -s "cargo screeps deploy -m ctf"

