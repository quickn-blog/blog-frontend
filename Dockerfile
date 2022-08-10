FROM rust

RUN mkdir /app
WORKDIR /app
COPY . /app
RUN rustup target add wasm32-unknown-unknown && \
    cargo install wasm-bindgen-cli --vers 0.2.71 && \
    cargo install trunk --vers 0.8.2
ENTRYPOINT ["trunk", "serve"]
