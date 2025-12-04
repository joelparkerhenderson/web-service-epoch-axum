FROM rust:1.89.0
WORKDIR /app
COPY . .
RUN cargo build --release
EXPOSE 8080
CMD ["./target/release/web-service-epoch-axum"]
