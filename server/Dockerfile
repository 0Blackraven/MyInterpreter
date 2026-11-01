# Stage 1: Build Rust interpreter
FROM rust:1.80-slim AS rust-builder
WORKDIR /app
COPY interpreter/server/interpreter/Cargo.toml interpreter/server/interpreter/Cargo.lock ./
COPY interpreter/server/interpreter/src ./src
RUN cargo build --release

# Stage 2: Build Node server
FROM node:20-slim
WORKDIR /app
RUN apt-get update && apt-get install -y build-essential && rm -rf /var/lib/apt/lists/*

# Copy Rust binary from builder
COPY --from=rust-builder /app/target/release/interpreter interpreter/server/target/release/interpreter

# Copy Node source
COPY interpreter/server/express/package*.json interpreter/server/express/
WORKDIR /app/interpreter/server/express
RUN npm install
COPY interpreter/server/express/ .

# Build Express/TypeScript
RUN npm run build

EXPOSE 3001
CMD ["npm", "start"]