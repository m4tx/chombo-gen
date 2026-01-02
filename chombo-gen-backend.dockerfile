FROM rust:1.92 as builder
WORKDIR /usr/src/chombo-gen
COPY . .
WORKDIR /usr/src/chombo-gen/chombo-gen-backend
RUN cargo install --path . --locked

FROM debian:13-slim
COPY --from=builder /usr/local/cargo/bin/chombo-gen-backend /usr/local/bin/chombo-gen-backend
CMD ["chombo-gen-backend", "-l", "0.0.0.0:8000"]
