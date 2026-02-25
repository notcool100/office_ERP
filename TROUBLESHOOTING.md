# Troubleshooting Guide — Ubuck ERP Deployment

## Architecture Overview

```
Browser → :3110 (Nginx) → / → :3111 (SvelteKit frontend)
                         → /api/ → :3117 (Rust backend, /api prefix stripped)
```

- **Nginx** listens on port `3110` (public-facing)
- **SvelteKit** runs internally on port `3111`
- **Rust backend** runs on port `3117`
- API calls go through `/api/` — Nginx strips the prefix before forwarding

---

## Common Issues

### 1. CORS Errors / "Status code: (null)"

**Symptom:** Browser console shows `Cross-Origin Request Blocked` with `Status code: (null)`

**Cause:** Frontend is calling the backend directly (different port = different origin). Browsers block this, especially on HTTP pages with password fields.

**Fix:** Ensure `frontend.env` uses a relative path, NOT a direct URL:

```diff
- PUBLIC_API_URL=http://82.180.144.91:3117
+ PUBLIC_API_URL=/api
```

> ⚠️ `PUBLIC_API_URL` is baked at build time (`$env/static/public`). You must **rebuild and redeploy** the frontend after changing it.

---

### 2. 502 Bad Gateway

**Symptom:** Nginx returns `502 Bad Gateway`

**Cause:** Nginx can't reach the upstream service. Common reasons:
- Frontend/backend crashed
- `localhost` resolves to IPv6 (`::1`) but the app only listens on IPv4

**Debug:**

```bash
# Check if services are running
sudo systemctl status ubuck-erp-frontend --no-pager
sudo systemctl status ubuck-erp-backend --no-pager

# Check which ports are actually listening
sudo ss -tlnp | grep -E '3110|3111|3117'

# Check logs
sudo journalctl -u ubuck-erp-frontend -n 30 --no-pager
sudo journalctl -u ubuck-erp-backend -n 30 --no-pager
```

**Fix:** Use `127.0.0.1` instead of `localhost` in Nginx config:

```nginx
proxy_pass http://127.0.0.1:3111;  # NOT localhost
```

---

### 3. "Unable to Connect" (site completely down)

**Symptom:** Browser can't connect at all

**Debug:**

```bash
# Is Nginx running and listening on 3110?
sudo ss -tlnp | grep 3110

# Is the config valid?
sudo nginx -t

# Is the site enabled?
ls -la /etc/nginx/sites-enabled/ubuck-erp
```

**Fix:**

```bash
sudo ln -sf /etc/nginx/sites-available/ubuck-erp /etc/nginx/sites-enabled/ubuck-erp
sudo nginx -t && sudo systemctl reload nginx
```

---

### 4. Frontend Changes Not Reflecting

**Symptom:** You updated code/env but nothing changed on the site

**Cause:** SvelteKit `$env/static/public` variables are baked at build time.

**Fix:** Rebuild and redeploy (push to main to trigger Azure pipeline, or manually):

```bash
# On VPS, after deploying new build:
sudo systemctl restart ubuck-erp-frontend
```

---

### 5. `systemctl edit` Fails with Terminal Error

**Symptom:** `Error opening terminal: xterm-kitty`

**Fix:**

```bash
export TERM=xterm
# Then retry, OR create the override file directly:
sudo mkdir -p /etc/systemd/system/ubuck-erp-frontend.service.d
sudo bash -c 'cat > /etc/systemd/system/ubuck-erp-frontend.service.d/override.conf << EOF
[Service]
Environment=PORT=3111
EOF'
sudo systemctl daemon-reload
```

---

## Quick Health Check Script

Run this to check everything at once:

```bash
echo "=== Nginx ===" && sudo nginx -t 2>&1 | tail -1
echo "=== Ports ===" && sudo ss -tlnp | grep -E '3110|3111|3117'
echo "=== Frontend ===" && sudo systemctl is-active ubuck-erp-frontend
echo "=== Backend ===" && sudo systemctl is-active ubuck-erp-backend
echo "=== Nginx ===" && sudo systemctl is-active nginx
echo "=== API Health ===" && curl -s http://127.0.0.1:3117/ | head -1
echo "=== Frontend Health ===" && curl -s -o /dev/null -w "%{http_code}" http://127.0.0.1:3111/
```

---

## Restart Everything

```bash
sudo systemctl restart ubuck-erp-backend
sudo systemctl restart ubuck-erp-frontend
sudo nginx -t && sudo systemctl reload nginx
```

---

## Key Files on VPS

| File | Purpose |
|------|---------|
| `/etc/nginx/sites-available/ubuck-erp` | Nginx reverse proxy config |
| `/etc/systemd/system/ubuck-erp-frontend.service` | Frontend systemd service |
| `/etc/systemd/system/ubuck-erp-frontend.service.d/override.conf` | PORT=3111 override |
| `/etc/systemd/system/ubuck-erp-backend.service` | Backend systemd service |
| `/var/www/ubuck-erp/fe/` | Deployed frontend files |
| `/var/www/ubuck-erp/fe/.env` | Frontend runtime env |
| `/var/www/ubuck-erp/be/` | Deployed backend binary + .env |
