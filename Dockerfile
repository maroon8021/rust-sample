FROM rustlang/rust:nightly

ENV USER=userName
WORKDIR /src
RUN cargo install cargo-watch