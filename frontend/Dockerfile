# ==========================================
# Stage 1: The Builder
# ==========================================
FROM rust:1.96-bookworm AS builder
RUN apt-get update && apt-get install -y curl pkg-config libssl-dev
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
WORKDIR /app
COPY package*.json ./
RUN npm install
COPY Cargo.toml Cargo.lock ./
COPY src src
COPY style style
COPY index.html index.html
RUN npx @tailwindcss/cli -i ./style/input.css -o ./style/output.css
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/app/target \
    trunk build --release

# ==========================================
# Stage 2: The Production Runner
# ==========================================
FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
