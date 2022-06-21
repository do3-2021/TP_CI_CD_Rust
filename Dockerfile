FROM alpine:3.16

LABEL MAINTAINER="do3 <do3@etu.umontpellier.fr>"
LABEL APP="city-api"

RUN addgroup --system --gid 1001 rust
RUN adduser --system --uid 1001 rust

WORKDIR /
COPY --chown=rust:rust target/release/city-api /usr/local/bin/city-api

USER rust

EXPOSE 2022
ENTRYPOINT ["city-api"]
