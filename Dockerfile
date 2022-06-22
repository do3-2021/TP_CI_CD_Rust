FROM ekidd/rust-musl-builder:stable as build

COPY . .
RUN USER=root cargo build --release

FROM alpine

LABEL MAINTAINER="do3 <do3@etu.umontpellier.fr>"
LABEL APP="city-api"

WORKDIR /
COPY --from=build /home/rust/src/target/x86_64-unknown-linux-musl/release/city-api /usr/local/bin/city-api

EXPOSE 2022
ENTRYPOINT ["city-api"]
