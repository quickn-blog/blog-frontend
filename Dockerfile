FROM rust

RUN mkdir /app
WORKDIR /app
COPY . /app
RUN rustup target add wasm32-unknown-unknown && \
    wget -qO- https://github.com/thedodd/trunk/releases/download/v0.10.0/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf- && \
    mv trunk /usr/bin && \
    wget -qO- https://github.com/rustwasm/wasm-bindgen/releases/download/0.2.72/wasm-bindgen-0.2.72-x86_64-unknown-linux-musl.tar.gz | tar -xzf- && \
    mv wasm-bindgen-0.2.72-x86_64-unknown-linux-musl/wasm-bindgen /usr/bin
CMD ["trunk", "serve"]