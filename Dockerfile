FROM ekidd/rust-musl-builder AS build
WORKDIR /usr/src/
USER root

# Add compilation target for later scratch container
ENV RUST_TARGETS="x86_64-unknown-linux-musl"
# install rustup/cargo
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH /root/.cargo/bin:$PATH
RUN rustup target install x86_64-unknown-linux-musl

# Creating a placeholder project
RUN USER=root cargo new cultist-gql-server
WORKDIR /usr/src/cultist-gql-server

# moving deps info
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Caching deps
RUN cargo build --target x86_64-unknown-linux-musl --release
RUN rm target/x86_64-unknown-linux-musl/release/deps/cult*

# Replacing with actual src
RUN rm src/*.rs
COPY ./src ./src

# Only code changes should need to compile
RUN cargo build --target x86_64-unknown-linux-musl --release

RUN ls /usr/src/cultist-gql-server/target/

# This creates a TINY container with the executable! Like 4-5mb srsly
FROM scratch
COPY --from=build /usr/src/cultist-gql-server/target/x86_64-unknown-linux-musl/release/cultist-gql-server .
USER 1000
CMD ["./cultist-gql-server"]
