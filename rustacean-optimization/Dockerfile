FROM rust:1.79.0-alpine AS build
# Install missing dependencies because of alpine
RUN apk add --no-cache musl-dev build-base
WORKDIR /usr/src/myapp
COPY . .
# Add target for building on alpine also added cc build-depenencies in cargo.toml
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:latest AS final
# Copy from the correct target directory in the build stage
COPY --from=build /usr/src/myapp/target/x86_64-unknown-linux-musl/release/rustacean-optimization /usr/local/bin/
WORKDIR /usr/local/bin/

# DEBUG: Check for missing libraries
# RUN ldd rustacean-optimization
# Enable dynamic linker tracing
# ENV LD_TRACE_LOADED_OBJECTS=1
CMD ["rustacean-optimization"]