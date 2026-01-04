# Deploying Ubuck ERP on VPS

This document outlines the steps to prepare your VPS and Azure DevOps environment for the Ubuck ERP application (Rust backend + SvelteKit frontend).

## 1. Azure DevOps Setup

### Secure Files
1. Go to your Azure DevOps Project -> **Pipelines** -> **Library**.
2. Click on the **Secure files** tab.
3. Upload the following files:
   - `backend.env` (for Rust backend)
   - `frontend.env` (for SvelteKit frontend)
4. **Important**: Click on each file after uploading, go to "Pipeline permissions", and "Authorize for use in all pipelines".

## 2. VPS Server Setup

### Directories
SSH into your VPS and create the necessary directories:

```bash
# Create main directory
sudo mkdir -p /var/www/ubuck-erp

# Create Backend and Frontend directories
sudo mkdir -p /var/www/ubuck-erp/be
sudo mkdir -p /var/www/ubuck-erp/fe

# Set ownership to your user initially (for manual setup) or www-data
sudo chown -R www-data:www-data /var/www/ubuck-erp
sudo chmod -R 755 /var/www/ubuck-erp
```

### System Dependencies
Ensure you have Nginx, Node.js, and other basics installed.

```bash
sudo apt update
sudo apt install -y nginx nodejs npm
sudo npm install -g pnpm
```

### Systemd Services

#### Backend Service
Create a file `/etc/systemd/system/ubuck-erp-backend.service`:

```ini
[Unit]
Description=Ubuck ERP Backend (Rust)
After=network.target

[Service]
User=www-data
Group=www-data
WorkingDirectory=/var/www/ubuck-erp/be
# Assuming the binary name is 'be'
ExecStart=/var/www/ubuck-erp/be/be
Restart=always
# The pipeline copies backend.env to .env in this directory
EnvironmentFile=/var/www/ubuck-erp/be/.env

[Install]
WantedBy=multi-user.target
```

#### Frontend Service
Create a file `/etc/systemd/system/ubuck-erp-frontend.service`:

```ini
[Unit]
Description=Ubuck ERP Frontend (SvelteKit Node Adapter)
After=network.target

[Service]
User=www-data
Group=www-data
WorkingDirectory=/var/www/ubuck-erp/fe
# node_modules and build folder are in this directory
ExecStart=/usr/bin/node build/index.js
Restart=always
# The pipeline copies frontend.env to .env in this directory
EnvironmentFile=/var/www/ubuck-erp/fe/.env
# Fallback env vars if needed
Environment=PORT=3000
Environment=ORIGIN=http://localhost:3000

[Install]
WantedBy=multi-user.target
```

Enable and start temporary (if you had files, otherwise the pipeline will start them):

```bash
sudo systemctl daemon-reload
```

## 3. Nginx Configuration

Create or edit your site config (e.g., `/etc/nginx/sites-available/default` or a new file):

```nginx
server {
    listen 80;
    server_name your-domain.com; # Relace with your IP or Domain

    # Frontend (SvelteKit)
    location / {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }

    # Backend (Rust API)
    location /api {
        proxy_pass http://localhost:8080;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }
}
```

Check and reload Nginx:
```bash
sudo nginx -t
sudo systemctl reload nginx
```
