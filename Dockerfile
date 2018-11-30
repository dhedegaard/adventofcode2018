FROM rust:latest

COPY . ./

CMD ["cargo", "test"]
