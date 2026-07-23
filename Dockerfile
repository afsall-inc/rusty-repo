FROM rust:1.84-slim-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --package rusty-repo-cli && \
    cp target/release/rusty-repo /usr/local/bin/rusty-repo

FROM gcr.io/distroless/cc-debian12:latest
COPY --from=builder /usr/local/bin/rusty-repo /usr/local/bin/rusty-repo
COPY --from=builder /app/templates /templates
ENTRYPOINT ["rusty-repo"]