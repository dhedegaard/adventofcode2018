FROM rust:slim AS BUILD
COPY . ./

RUN cargo test -q --all --release
RUN cargo build --release

FROM debian:9-slim

COPY --from=BUILD target/release/adventofcode2018 /
CMD ["/adventofcode2018"]
