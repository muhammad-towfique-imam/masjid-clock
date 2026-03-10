# Masjid Clock

A digital display system for prayer times, designed to be deployed as a web application.

## Local Development

To run the development server with automatic reloading:

```bash
# Install cargo-watch if you don't have it
cargo install cargo-watch

# Run the development server
cargo watch -x run
```

The server will be available at `http://localhost:8000`.

## Building for Release

To create an optimized release build:

```bash
cargo build --release
```

The resulting binary will be located at `target/release/masjid-clock`.

## Deployment to Fly.io

### 1. Initial Deployment
Ensure you have the [Fly CLI](https://fly.io/docs/hands-on/install-flyctl/) installed and are logged in.

```bash
fly deploy
```

### 2. Custom Domain & SSL Certificates

To set up a custom domain (e.g., `masjid-clock.cyanwarelabs.com`):

#### Add the Certificate
```bash
fly certs add masjid-clock.cyanwarelabs.com
```

#### Configure DNS
Add the IPv4 and IPv6 addresses to your DNS provider (e.g., Cloudflare):
1. Run `fly ips list` to get your app's IP addresses.
2. Create **A** and **AAAA** records pointing to these IPs.
3. Keep Proxy status as **DNS Only (Grey Cloud)** during verification.

#### Verify the Certificate
Check the status of your certificate and ownership verification:
```bash
fly certs show masjid-clock.cyanwarelabs.com
```

If ownership verification is required, Fly will provide a `_fly-ownership` TXT record value. Add this to your DNS settings.

### 3. Cloudflare Proxying (Optional)
Once the certificate is "Verified" in Fly.io:
1. Change your DNS records in Cloudflare to **Proxied (Orange Cloud)**.
2. Set SSL/TLS mode to **Full (Strict)** in the Cloudflare dashboard.

## Tech Stack
- **Backend:** Rust with [Rocket](https://rocket.rs/)
- **Templates:** [Tera](https://keats.github.io/tera/)
- **Frontend:** Tailwind CSS, Alpine.js, Luxon
