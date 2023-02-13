FROM rust:1.67

WORKDIR /usr/src/webprogrammierung_group_5
COPY . .

RUN rustup default nightly

EXPOSE 8000

CMD cargo run