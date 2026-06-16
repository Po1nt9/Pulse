# Pulse Development Environment
# Docker is for frontend tooling and non-GUI validation only.
# Build and run Tauri GUI apps natively on Windows/macOS/Linux.
FROM node:20-slim

# Install Rust and common build dependencies used by Tauri/Rust crates.
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    libssl-dev \
    pkg-config \
    libgtk-3-dev \
    libwebkit2gtk-4.1-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Rust via rustup.
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Pin the Tauri CLI version range so container builds are reproducible.
RUN cargo install tauri-cli --version "^2.0.0"

WORKDIR /app

# Copy the repository. The project may still be pre-implementation, so dependency
# installation is conditional until package.json/src-tauri/Cargo.toml exist.
COPY . .
RUN if [ -f package.json ]; then npm install; fi
RUN if [ -f src-tauri/Cargo.toml ]; then cd src-tauri && cargo fetch; fi

EXPOSE 1420

CMD ["sh", "-c", "if [ -f package.json ]; then npm run dev -- --host 0.0.0.0; else echo 'Pulse is not initialized yet. Run the implementation plan first.'; sleep infinity; fi"]
