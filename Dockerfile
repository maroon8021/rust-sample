FROM rustlang/rust:nightly

ENV USER=userName
WORKDIR /src
RUN apt-get update