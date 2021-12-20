FROM ubuntu:20.04
ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y ca-certificates dumb-init pkg-config python3 && rm -rf /var/lib/apt/lists/*
ENTRYPOINT ["/usr/bin/dumb-init","--"]
ARG profile=release
EXPOSE 80
COPY \
   . \
   /usr/local/bin/
