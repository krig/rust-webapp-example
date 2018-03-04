FROM rust:latest
MAINTAINER <krig@koru.se>

WORKDIR /usr/src/webapp/
COPY . .

RUN cargo install

CMD ["rust-webapp-example"]
