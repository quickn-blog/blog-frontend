FROM rust

RUN mkdir /app
WORKDIR /app
COPY . /app
RUN cargo install wasm-bindgen-cli
RUN cargo install --locked trunk
CMD ["trunk", "serve"]