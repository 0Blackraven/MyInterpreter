# Stage 1: Build Rust interpreter
FROM rust:1.80-slim AS rust-builder
WORKDIR /app
COPY server/interpreter/Cargo.toml server/interpreter/Cargo.lock ./
COPY server/interpreter/src ./src
RUN cargo build --release

# Stage 2: Build Node server
FROM node:20-slim
WORKDIR /app
RUN apt-get update && apt-get install -y build-essential && rm -rf /var/lib/apt/lists/*

# Copy Rust binary from builder
COPY --from=rust-builder /app/target/release/interpreter ./server/interpreter/target/release/interpreter

# Copy Node source
COPY server/express/package*.json ./server/express/
WORKDIR /app/server/express
RUN npm install
COPY server/express/ .

# Build Express/TypeScript
RUN npm run build

EXPOSE 3001
CMD ["npm", "start"]