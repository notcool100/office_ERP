# Coolify Deployment Guide

This repo now includes Docker deployment support for Coolify:
- `be/Dockerfile` for the Rust backend
- `fe/Dockerfile` for the SvelteKit frontend

## 1. Deploy the database

Use Coolify to create a managed PostgreSQL database.

Required backend environment variables:
- `DATABASE_URL` = PostgreSQL connection string
- `GITHUB_TOKEN_ENC_KEY` = 32-byte base64 key used to encrypt the GitHub connector's stored token (generate with `openssl rand -base64 32`)
- optional: `MAILCOW_API_URL`, `MAILCOW_API_KEY`, `MAILCOW_MAIL_DOMAIN` = Mailcow REST API mailbox provisioning
- optional: `FCM_SERVICE_ACCOUNT_JSON` = the full contents of a Firebase service-account key (Firebase console → Project settings → Service accounts → Generate new private key), minified to one line, pasted directly as the env var's value — enables push notifications to the mobile app. `FCM_PROJECT_ID` is read from the key itself and doesn't need to be set separately. Without this, the backend runs normally and the mobile app's notification list still works over REST/WebSocket; phones just won't get woken up while backgrounded. See `mobile/README.md` for the mobile-side half of this setup.

Example PostgreSQL URL:
```text
postgres://user:password@your-host:5432/your_database
```

## 2. Deploy the backend app

Create a new Coolify app using the `be` folder as the build context.
- Build context: `be/`
- Dockerfile: `Dockerfile`
- Port: `3117`
- Environment variables:
  - `PORT=3117`
  - `DATABASE_URL`
  - `GITHUB_TOKEN_ENC_KEY` (required to use the GitHub connector — Settings > Connectors)
  - `MAILCOW_API_URL`, `MAILCOW_API_KEY`, `MAILCOW_MAIL_DOMAIN` (optional, for mailbox auto-provisioning)
  - `SMTP_HOST`, `SMTP_USERNAME`, `SMTP_PASSWORD`, `SMTP_PORT` (if mail is enabled)
  - `FCM_SERVICE_ACCOUNT_JSON` (optional, for mobile push notifications — see above)

The backend listens on `0.0.0.0:3117` by default.

**Persistent storage:** in the Coolify app's Storage tab, add a persistent
volume mounted at `/app/uploads`. This is where uploaded company branding
(logo/favicon) is written. Without it, every redeploy starts from a fresh
container filesystem, `company_settings` keeps pointing at files that no
longer exist, and the logo/favicon requests 404 (which browsers report as
"blocked by OpaqueResponseBlocking" since the 404 body isn't image data).

## 3. Deploy the frontend app

Create a second Coolify app using the `fe` folder.
- Build context: `fe/`
- Dockerfile: `Dockerfile`
- Port: `3110`
- Environment variables:
  - `PORT=3110`
  - `PUBLIC_API_URL`

### Preferred API routing options

Option A — backend on a separate domain/subdomain:
- Set `PUBLIC_API_URL=https://api.example.com`
- Deploy backend to `api.example.com`
- Deploy frontend to `app.example.com` or `example.com`

Option B — path-based proxy (if Coolify supports `/api` routing for your frontend domain):
- Set `PUBLIC_API_URL=/api`
- Route frontend requests to the backend for paths under `/api`

> Note: `PUBLIC_API_URL` is baked into the frontend at build time. If you change it, rebuild and redeploy the frontend.

## 4. CORS and network notes

The backend already enables CORS for all origins. If you use separate subdomains, the frontend can call the backend directly.

## 5. Local Docker-based testing (optional)

You can test the services locally before deploying:

```yaml
version: '3.9'
services:
  db:
    image: postgres:16
    environment:
      POSTGRES_USER: office
      POSTGRES_PASSWORD: office
      POSTGRES_DB: office
    ports:
      - 5432:5432
    volumes:
      - db-data:/var/lib/postgresql/data

  backend:
    build:
      context: ./be
      dockerfile: Dockerfile
    ports:
      - 3117:3117
    environment:
      PORT: 3117
      DATABASE_URL: postgres://office:office@db:5432/office
    volumes:
      - uploads-data:/app/uploads
    depends_on:
      - db

  frontend:
    build:
      context: ./fe
      dockerfile: Dockerfile
    ports:
      - 3110:3110
    environment:
      PORT: 3110
      PUBLIC_API_URL: http://localhost:3117
    depends_on:
      - backend

volumes:
  db-data:
  uploads-data:
```

## 6. What was added

- `be/Dockerfile`
- `fe/Dockerfile`
- `be/.dockerignore`
- `fe/.dockerignore`
- `COOLIFY.md`

Use these files with Coolify to host the frontend, backend, and database cleanly.
