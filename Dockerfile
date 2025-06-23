FROM ubuntu:noble-20250529
WORKDIR /runtime
COPY backend/target/release/backend ./app
COPY frontend/build ./public
EXPOSE 3000
ENV RUST_LOG="info"
CMD ["./app"]
