FROM rust:slim AS rust-build

WORKDIR /build/interpreter

COPY server/interpreter/Cargo.toml ./
COPY server/interpreter/src ./src

RUN cargo build --release

FROM node:18-slim AS node-build

WORKDIR /build/express

COPY server/express/package*.json ./
RUN npm ci

COPY server/express/tsconfig.json ./
COPY server/express/src ./src

RUN npm run build

FROM node:18-slim

WORKDIR /app

# Copy built artifacts
COPY --from=rust-build /build/interpreter/target/release/interpreter ./server/interpreter/target/release/interpreter
COPY --from=node-build /build/express/dist ./server/express/dist
COPY --from=node-build /build/express/node_modules ./server/express/node_modules
COPY server/express/package.json ./server/express/package.json

WORKDIR /app/server/express

EXPOSE 3001

CMD ["node", "dist/index.js"]