FROM rust
WORKDIR /service1
COPY . /service1
EXPOSE 80
RUN cargo build --release
CMD ["cargo", "run", "--release"]