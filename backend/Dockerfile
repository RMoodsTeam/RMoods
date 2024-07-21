ARG RUST_VERSION=1.79.0
ARG APP_NAME=rmoods-backend

# Create a stage for building the application.
FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app

# Install host build dependencies.
RUN apk add --no-cache clang lld musl-dev git libressl-dev curl

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
cargo build --locked --release && \
cp ./target/release/$APP_NAME /app/server

# Create a new stage, to discard all unncessary files
FROM alpine:3.18 AS final

ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

# Set the working directory.
WORKDIR /app

# Copy the executable from the "build" stage.
COPY --from=build /app/server /app/

# Copy the .env file from the build context to the final stage.
COPY .env /app/.env

# Expose the port that the application listens on.
EXPOSE 8001

# What the container should run when it is started.
CMD ["/app/server"]
