FROM rust

RUN mkdir /app
WORKDIR /app
COPY . /app
RUN rustup target add wasm32-unknown-unknown && \
    cargo install wasm-bindgen-cli && \
    cargo install trunk
ENTRYPOINT ["trunk", "serve"]
