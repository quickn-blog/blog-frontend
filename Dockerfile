FROM rust

RUN mkdir /app
WORKDIR /app
COPY . /app
RUN rustup target add wasm32-unknown-unknown && \
    wget -qO- https://github.com/rustwasm/wasm-bindgen/releases/download/0.2.72/wasm-bindgen-0.2.72-x86_64-unknown-linux-musl.tar.gz | tar -xzf- && \
    mv wasm-bindgen-0.2.72-x86_64-unknown-linux-musl/wasm-bindgen /usr/bin
RUN cargo install --locked trunk
CMD ["trunk", "serve"]