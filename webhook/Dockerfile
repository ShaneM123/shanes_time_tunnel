FROM almir/webhook

RUN mkdir -p /root/.ssh/ && echo \
  "github.com ssh-rsa AAAAB3NzaC1yc2EAAAABIwAAAQEAq2A7hRGmdnm9tUDbO9IDSwBK6TbQa+PXYPCPy6rbTrTtw7PHkccKrpp0yVhp5HdEIcKr6pLlVDBfOLX9QUsyCOV0wzfjIJNlGEYsdlLJizHhbn2mUjvSAHQqZETYP81eFzLQNnPHt4EVVUh7VfDESU84KezmD5QlWpXLmvU31/yMf+Se8xhHTvKSCZIFImWwoG6mbUoWf9nzpIoaSjB+weqqUUmpaaasXVal72J+UX2B+2RPW3RcT0eOzQgqlJL3RKrTJvdsjE3JEAvGq3lGHSZXy28G3skua2SmVi/w4yCE6gbODqnTWlg7+wC604ydGXA8VJiS5ap43JXiUFFAaQ==" \
  > /root/.ssh/known_hosts

COPY hook.json /hook.json
ARG SECRET_WEBHOOK
RUN sed -i "s|\$SECRET_WEBHOOK|$SECRET_WEBHOOK|g" /hook.json
CMD ["-verbose", "-hooks=/hook.json"]

ARG DOCKER_REGISTRY=ghcr.io
ARG DOCKER_USERNAME
ARG DOCKER_TOKEN
ENV DOCKER_REGISTRY=$DOCKER_REGISTRY DOCKER_USERNAME=$DOCKER_USERNAME DOCKER_TOKEN=$DOCKER_TOKEN
RUN apk add git openssh-client docker docker-compose
COPY deploy.sh /deploy.sh