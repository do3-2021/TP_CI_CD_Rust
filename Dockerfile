FROM scratch:latest

WORKDIR /
COPY target/release/rust-tp-cicd ./app

EXPOSE 2022
ENTRYPOINT [ "/app" ]
