FROM rust:1.62 as builder
WORKDIR /usr/src/myapp
COPY . .
ARG github_token 
RUN git config --global credential.helper store && echo "https://zefanjajobse:${github_token}@github.com" > ~/.git-credentials && cargo install --path .

FROM debian:bullseye

HEALTHCHECK --interval=5m --timeout=3s --start-period=5s \
    CMD curl -f http://127.0.0.1:3030/ || exit 1

COPY --from=builder /usr/local/cargo/bin/discord_playerlist /usr/local/bin/discord_playerlist
RUN apt-get update && apt-get install --assume-yes curl
CMD ["discord_playerlist"]
