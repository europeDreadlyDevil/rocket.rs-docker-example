FROM rust
WORKDIR /service2
COPY . /service2
EXPOSE 80
RUN cargo build --release
CMD ["cargo", "run", "--release"]