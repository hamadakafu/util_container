FROM rust:alpine3.10 as builder

ARG PROJECT_NAME=util_container
WORKDIR /workspace/${PROJECT_NAME}

# https://github.com/rust-lang/cargo/issues/7563
# for proc-macro
ENV RUSTFLAGS='-C target-feature=-crt-static'
RUN apk add --no-cache g++

RUN mkdir src && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

COPY Cargo.toml Cargo.lock ./

RUN cargo build --release
RUN rm -rf target/release/${PROJECT_NAME}
RUN rm -f target/release/deps/${PROJECT_NAME}*
RUN rm -rf src/

COPY src src
RUN cargo build --release
# CMD ["cargo", "run", "--release"]


# FROM gcr.io/distroless/static:latest
FROM alpine:3.10

ARG PROJECT_NAME=util_container

RUN apk add --no-cache g++

USER nobody
WORKDIR /workspace

COPY --from=builder /workspace/${PROJECT_NAME}/target/release/${PROJECT_NAME} /workspace/app

CMD ["/workspace/app"]
