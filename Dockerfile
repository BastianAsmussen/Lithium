FROM rust:latest

WORKDIR /usr/src/lithium
COPY . .

RUN cargo install --path .

CMD ["lithium", "examples/test.lt"]
