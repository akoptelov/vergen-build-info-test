FROM rust:buster AS build
WORKDIR /build
COPY . .
RUN ls -al
RUN cargo build --release

FROM debian:buster
COPY --from=build /build/target/release/vergen-build-info-test /usr/local/bin/
RUN /usr/local/bin/vergen-build-info-test
ENTRYPOINT [ "vergen-build-info-test" ]
