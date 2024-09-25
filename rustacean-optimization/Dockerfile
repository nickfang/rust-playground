FROM rust:latest AS build
WORKDIR /usr/src/myapp
COPY . .
RUN cargo build --release

FROM rust:latest AS final
COPY --from=build /usr/src/myapp/target/release/rustacean-optimization /usr/local/bin/rustacean-optimization
CMD ["rustacean-optimization"]