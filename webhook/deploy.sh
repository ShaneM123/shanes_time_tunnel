#!/bin/sh
set -e

cd /roguelike2021

BRANCH="origin/main"
REGISTRY="ghcr.io"

git fetch

git reset --hard "$BRANCH"

docker login "$REGISTRY" -u "$DOCKER_USERNAME" --password "$DOCKER_TOKEN"

docker-compose -f docker-compose.yml -p shanes_time_tunnel pull
docker-compose -f docker-compose.yml -p shanes_time_tunnel build --pull
docker-compose -f docker-compose.yml -p shanes_time_tunnel up -d --remove-orphans

docker system prune --force --all