FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye 

# RUN apt update && apt install --no-install-recommends -y r-base
RUN curl -Ls https://github.com/r-lib/rig/releases/download/latest/rig-linux-latest.tar.gz | tar xz -C /usr/local
RUN rig add release
# 
# https://github.com/r-lib/rig/issues/162
# not ready yet