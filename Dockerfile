# syntax=docker/dockerfile:1
FROM rust:1.91 AS build

WORKDIR /app

COPY . .

RUN cargo build --release


FROM ubuntu:24.04 AS runtime

RUN groupadd -g 1001 appgroup && \
    useradd -u 1001 -g appgroup -m -d /home/appuser -s /bin/bash appuser

COPY --from=build --chown=appuser:appgroup /app/target/release/ryzom-patch-info /usr/local/bin/ryzom-patch-info
USER appuser
WORKDIR /app

ENTRYPOINT ["/usr/local/bin/ryzom-patch-info"]