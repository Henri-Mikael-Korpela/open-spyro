FROM alpine:3.19

RUN apk add --no-cache build-base
RUN apk add --no-cache curl
RUN apk add --no-cache musl-dev
RUN apk add --no-cache git

# Install Rust
RUN curl -proto '=https' -tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Ensure Cargo version 1.79 is installed
#RUN cargo --version | grep "^cargo 1.79" || exit 1

# Ensure Rust version 1.79 is installed
#RUN rustc --version | grep "^rustc 1.79" || exit 1

# Ensure Rustup version 1.27 is installed
#RUN rustup --version | grep "^rustup 1.27" || exit 1

# Install Python 3
RUN apk add --no-cache python3

# Ensure Python version 3.11 is installed
RUN python3 --version | grep "^Python 3.11" || exit 1

WORKDIR /main

# Keep the container running
CMD ["tail", "-f", "/dev/null"]