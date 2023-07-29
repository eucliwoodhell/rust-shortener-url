FROM debian:buster-slim

WORKDIR /app

ENTRYPOINT  [ "/app/entrypoint.sh" ]
