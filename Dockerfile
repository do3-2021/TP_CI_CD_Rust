FROM scratch

WORKDIR /
COPY target/release/rust-tp-cicd /usr/local/bin/rust-tp-cicd

EXPOSE 2022
ENTRYPOINT ["rust-tp-cicd"]
