FROM rust:1.68 as builder
WORKDIR /usr/src/chombo-gen
COPY . .
WORKDIR /usr/src/chombo-gen/backend
RUN cargo install --path . --locked

FROM debian:11-slim
COPY --from=builder /usr/local/cargo/bin/chombo-gen-backend /usr/local/bin/chombo-gen-backend
ENV ROCKET_ADDRESS=0.0.0.0
CMD ["chombo-gen-backend"]
