FROM rust:1.85 as builder
RUN rustup target add wasm32-unknown-unknown && \
     cargo install trunk
WORKDIR /usr/src/chombo-gen
COPY . .
WORKDIR /usr/src/chombo-gen/chombo-gen-frontend
ENV CHOMBO_GEN_API_URL=/api
RUN trunk build --release

FROM nginx:1.27
COPY --from=builder /usr/src/chombo-gen/chombo-gen-frontend/dist /usr/share/nginx/html
