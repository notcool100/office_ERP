# Coolify Deployment Guide

This repo now includes Docker deployment support for Coolify:
- `be/Dockerfile` for the Rust backend
- `fe/Dockerfile` for the SvelteKit frontend

## 1. Deploy the database

Use Coolify to create a managed PostgreSQL database.

Required backend environment variables:
- `DATABASE_URL` = PostgreSQL connection string
- optional: `VMAIL_DATABASE_URL` = MySQL connection string for the optional vmail pool

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
  - `VMAIL_DATABASE_URL` (optional)
  - `SMTP_HOST`, `SMTP_USERNAME`, `SMTP_PASSWORD`, `SMTP_PORT` (if mail is enabled)

The backend listens on `0.0.0.0:3117` by default.

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
      POSTGRES_USER: ubuck
      POSTGRES_PASSWORD: ubuck
      POSTGRES_DB: ubuck
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
      DATABASE_URL: postgres://ubuck:ubuck@db:5432/ubuck
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
```

## 6. What was added

- `be/Dockerfile`
- `fe/Dockerfile`
- `be/.dockerignore`
- `fe/.dockerignore`
- `COOLIFY.md`

Use these files with Coolify to host the frontend, backend, and database cleanly.
