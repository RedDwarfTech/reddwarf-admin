# to reduce the docker image size
# https://stackoverflow.com/questions/69144154/why-is-the-rust-docker-image-so-huge
# build stage
FROM rust:1.54-bullseye as builder
WORKDIR /app
COPY . /app
RUN rustup default stable
RUN cargo build --release
# RUN cargo build

# Prod stage
FROM debian:bullseye-slim
LABEL maintainer="jiangtingqiang@gmail.com"
WORKDIR /app
ENV ROCKET_ADDRESS=0.0.0.0
# ENV ROCKET_PORT=11014
# https://stackoverflow.com/questions/69153048/error-while-loading-shared-libraries-libpq-so-5-cannot-open-shared-object-file
# https://unix.stackexchange.com/questions/668754/what-is-libpq-so-5-and-how-to-make-it-avaliable/668755
RUN apt-get update && apt-get install libpq5 curl -y
COPY --from=builder /app/.env /app
COPY --from=builder /app/settings.toml /app
# COPY --from=builder /app/target/debug/* /app/
# 
# only copy the execute file to minimal the image size
# do not copy the release folder
COPY --from=builder /app/target/release/reddwarf-admin /app/
COPY --from=builder /app/Rocket.toml /app
CMD ["./reddwarf-admin"]



