#!/usr/bin/env bash

set -u

set -e

set -x

docker build -f ./Dockerfile.alpine -t=reddwarf-pro/reddwarf-admin:v1.0.0 .