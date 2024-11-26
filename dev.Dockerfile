FROM ubuntu:24.04

# Install dependencies
RUN apt-get update
RUN apt-get install -y build-essential curl git python3

# Install Rust
RUN curl -proto '=https' -tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Ensure correct Cargo, Python 3, Rust, rustup versions are installed
RUN (cargo --version | grep "^cargo 1.82" || exit 1) \
    && (python3 --version | grep "^Python 3.12" || exit 1) \
    && (rustc --version | grep "^rustc 1.82" || exit 1) \
    && (rustup --version | grep "^rustup 1.27" || exit 1)

WORKDIR /main

# Keep the container running
CMD ["tail", "-f", "/dev/null"]