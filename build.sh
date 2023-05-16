#!/bin/bash

podman build -t docker.io/nielswps/polkem-runner:latest . && \
podman push nielswps/polkem-runner