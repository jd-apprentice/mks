FROM rust:slim-bookworm AS build

RUN USER=root cargo new --bin mks
WORKDIR /tmp/app

RUN apt-get update && apt-get install -y \
    build-essential \
    ## Add additional dependencies here
    && rm -rf /var/lib/apt/lists/*

COPY . .
RUN cargo build --release --bin mks

FROM rust:slim-bookworm AS runtime

WORKDIR /app
COPY --from=build /tmp/app/target/release/mks /app/mks

RUN groupadd user \
&& useradd -m -g user user \
&& chown -R user:user /app

USER user
ENTRYPOINT ["/app/mks"]