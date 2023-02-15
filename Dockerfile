FROM rust:1.67 as builder
WORKDIR /usr/src/video-storage
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/video-storage /usr/local/bin/video-storage
RUN mkdir -p /data/files
WORKDIR /data
EXPOSE 8080
CMD ["video-storage"]
