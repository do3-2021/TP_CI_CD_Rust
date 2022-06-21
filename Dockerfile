FROM scratch

LABEL MAINTAINER="do3 <do3@etu.umontpellier.fr>"
LABEL APP="city-api"

RUN addgroup --system --gid 1001 rust
RUN adduser --system --uid 1001 rust

WORKDIR /
COPY --chown=rust:rust target/release/rust-tp-cicd /usr/local/bin/rust-tp-cicd

USER rust

EXPOSE 2022
ENTRYPOINT ["rust-tp-cicd"]
