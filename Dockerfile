FROM scratch

LABEL MAINTAINER="do3 <do3@etu.umontpellier.fr>"
LABEL APP="city-api"

WORKDIR /
COPY target/release/city-api /usr/local/bin/city-api

EXPOSE 2022
ENTRYPOINT ["city-api"]
