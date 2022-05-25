FROM scratch

WORKDIR /usr/local/bin
COPY target/release/rust-tp-cicd ./rust-tp-cicd

EXPOSE 2022
ENTRYPOINT ["app"]
