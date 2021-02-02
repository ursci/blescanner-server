# builder
FROM rust:1.43.1 as builder
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV APP_HOME /usr/src/blescanner

WORKDIR ${APP_HOME}
COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10

COPY --from=builder /usr/local/cargo/bin/blescanner /usr/local/bin/blescanner

CMD ["blescanner"]
