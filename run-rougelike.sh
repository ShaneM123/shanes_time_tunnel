#!/usr/bin/env bash
set -eu

docker-compose build --pull

docker-compose up --remove-orphans
