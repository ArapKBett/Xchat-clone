FROM rust:latest
WORKDIR /usr/src/xchat-clone
COPY . .
RUN cargo build --release
RUN mkdir -p uploads
EXPOSE 8080
CMD ["cargo", "run", "--release"]
