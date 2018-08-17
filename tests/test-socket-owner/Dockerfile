FROM rustlang/rust:nightly-slim

RUN useradd toby

COPY . /usr/src/toby
WORKDIR /usr/src/toby

RUN mkdir -p /usr/local/etc/toby/ && \
    touch /usr/local/etc/toby/toby.toml && \
    touch /usr/local/etc/toby/tokens.toml

RUN ./configure && cargo build --all-features

CMD /usr/src/toby/target/debug/toby-workerd