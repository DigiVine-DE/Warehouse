FROM clux/muslrust:1.60.0-nightly-2022-02-18 as rust-build
WORKDIR /usr/src/digivine-api
ADD  . .
RUN cargo build --release

FROM openapitools/openapi-generator-cli:v5.4.0 as openapi-build
ADD . .
RUN apt-get update; 	apt-get install -y --no-install-recommends ca-certificates openssl; rm -rf /var/lib/apt/lists/*
RUN mkdir -p ./openapi/bundled
RUN wget https://raw.githubusercontent.com/DigiVine-DE/API-specification/main/bundled/openapi.yaml -O ./openapi/bundled/openapi.yaml
RUN java -jar /opt/openapi-generator/modules/openapi-generator-cli/target/openapi-generator-cli.jar\
       generate \
       -i ./openapi/bundled/openapi.yaml \
       -g html2 \
       -o /build/html-documentation \
       --generate-alias-as-model
RUN java -jar /opt/openapi-generator/modules/openapi-generator-cli/target/openapi-generator-cli.jar\
       generate \
       -i ./openapi/bundled/openapi.yaml \
       -g openapi \
       -o /build/json 
RUN java -jar /opt/openapi-generator/modules/openapi-generator-cli/target/openapi-generator-cli.jar\
       generate \
       -i ./openapi/bundled/openapi.yaml \
       -g openapi-yaml \
       -o /build/yaml 

FROM nginx:stable-alpine3.17
RUN apk add --no-cache supervisor
COPY --from=rust-build /usr/src/digivine-api/target/x86_64-unknown-linux-musl/release/digivine-api /digivine-api
COPY --from=openapi-build /build/html-documentation /usr/share/nginx/html/doc
COPY --from=openapi-build /build/json/openapi.json  /usr/share/nginx/html/openapi/openapi.json 
COPY --from=openapi-build /build/yaml/openapi/openapi.yaml  /usr/share/nginx/html/openapi/openapi.yaml 
COPY ./docker/nginx.conf /etc/nginx/nginx.conf
COPY ./docker/start.sh  /start.sh
COPY ./docker/supervisor.conf /etc/supervisor/conf.d/supervisor.conf
RUN chmod +x /start.sh
CMD [ "/start.sh" ]
