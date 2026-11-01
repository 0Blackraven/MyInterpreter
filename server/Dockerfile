# Stage 1: Build Rust interpreter
FROM rust:1.80-slim AS rust-builder
WORKDIR /app
COPY ./interpreter/Cargo.toml ./
COPY ./interpreter/src ./src
RUN cargo build --release

# Stage 2: Build Node server
FROM node:20-slim
WORKDIR /app
RUN apt-get update && apt-get install -y build-essential && rm -rf /var/lib/apt/lists/*

# Copy Rust binary from builder
COPY --from=rust-builder /app/target/release/interpreter /app/interpreter

# Copy Node source and install deps
COPY ./express/package*.json ./express/
WORKDIR /app/express
RUN npm install
COPY ./express/ .

# Build Express/TypeScript
RUN npm run build

ENV INTERPRETER_BIN=/app/interpreter
ENV PORT=3001

EXPOSE 3001
CMD ["npm", "start"]