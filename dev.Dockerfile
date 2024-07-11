FROM alpine:3.19

# Install Git
RUN apk add --no-cache git

# Install Rust, Cargo and Rustup
RUN apk add --no-cache rust cargo rustup

# Install Python 3
RUN apk add --no-cache python3

# Ensure Python version 3.11 is installed
RUN python3 --version | grep "^Python 3.11" || exit 1

WORKDIR /main

# Keep the container running
CMD ["tail", "-f", "/dev/null"]