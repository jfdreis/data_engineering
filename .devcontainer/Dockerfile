# Use official VS Code base image for dev containers
FROM mcr.microsoft.com/vscode/devcontainers/base:ubuntu

# Install Rust non-interactively
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Add Cargo to PATH for all shells
ENV PATH="/root/.cargo/bin:${PATH}"

# Use bash for all container shell commands
SHELL ["/bin/bash", "-c"]
