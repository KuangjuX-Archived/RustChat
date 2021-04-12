FROM rust:latest
WORKDIR /app

COPY . .

EXPOSE 8088

RUN cd /app/chatroom && \
    chmod +x ./src && \
    cargo run --bin server