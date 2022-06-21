FROM rust:latest as build

WORKDIR /
COPY . .
RUN cargo build --release

FROM scratch

LABEL MAINTAINER="do3 <do3@etu.umontpellier.fr>"
LABEL APP="city-api"

WORKDIR /
COPY --from=build target/release/city-api /usr/local/bin/city-api

EXPOSE 2022
ENTRYPOINT ["city-api"]
