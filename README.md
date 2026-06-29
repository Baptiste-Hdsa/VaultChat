# VaultChat

A self-hosted, end-to-end encrypted messaging platform built entirely in Rust.

VaultChat utilizes a "blind" backend architecture. All cryptographic operations including RSA key generation, AES-GCM message encryption, and key vaulting are executed strictly client-side within a secure WebAssembly (WASM) sandbox. The server routes data but never has access to raw private keys or plaintext messages.

## Architecture
* **Frontend:** Rust / Leptos (WASM) / Tailwind CSS (DaisyUI)
* **Backend:** Rust / Axum / SQLx
* **Database:** PostgreSQL
* **Infrastructure:** Docker / Nginx Reverse Proxy

## Deployment

VaultChat is fully containerized. To deploy the instance:

1. **Clone the repository:**
   git clone https://github.com/Baptiste-Hdsa/VaultChat.git
   cd VaultChat

2. **Environment Variables:**
   Ensure your .env file is properly configured with your PostgreSQL credentials and server settings (don't use the defaults ones given, change them).

3. **Build and Start:**
   Run the full stack (Frontend, Backend, and Database) via Docker Compose:
   docker compose up -d --build

   *(Note: Database migrations via SQLx run automatically on backend startup.)*

## Connecting to the Site

**Local Development:**
The frontend container maps to port 8584 by default.
* Navigate to: http://localhost:8584
* The application automatically uses standard ws:// for local real-time connections.

**Production:**
In a production environment, VaultChat is designed to sit behind a host-level Nginx reverse proxy handling SSL termination.
* Navigate to your secured domain (e.g., https://yourdomain.com).
* The client will automatically upgrade to secure WebSockets (wss://).
