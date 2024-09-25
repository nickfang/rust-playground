FROM rust:latest AS build
WORKDIR /usr/src/myapp
COPY . .

FROM
RUN cargo build --release
FROM debian:buster-slim AS final
COPY --from=build /usr/src/myapp/target/release/rustacean-optimization /usr/local/bin/rustacean-optimization
CMD ["rustacean-optimization"]