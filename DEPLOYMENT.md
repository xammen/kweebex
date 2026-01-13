# Kweebex Deployment Guide

## Quick Start with Coolify

### Option 1: Docker Compose (Recommended)

1. In Coolify, create a new project
2. Add a new resource > Docker Compose
3. Connect your GitHub repo: `https://github.com/xammen/kweebex`
4. Set the compose file to: `docker-compose.coolify.yml`
5. Add these environment variables:

```env
POSTGRES_PASSWORD=generate-secure-password-here
MEILISEARCH_MASTER_KEY=generate-another-key-here
SITE_URL=https://kweebex.com
API_URL=https://api.kweebex.com
LABRINTH_ADMIN_KEY=generate-admin-key-here
RATE_LIMIT_IGNORE_KEY=generate-rate-key-here
```

6. Deploy!

### Option 2: Individual Services

Deploy each service separately in Coolify:

#### 1. PostgreSQL
- Add Database > PostgreSQL
- Note the connection string

#### 2. Redis  
- Add Database > Redis
- Note the connection string

#### 3. Meilisearch
- Add Resource > Docker Image
- Image: `getmeili/meilisearch:v1.6`
- Environment:
  - `MEILI_MASTER_KEY=your-master-key`
  - `MEILI_NO_ANALYTICS=true`

#### 4. Labrinth (API)
- Add Resource > Dockerfile
- Dockerfile path: `Dockerfile.labrinth`
- Port: 8000
- See `.env.coolify.example` for required variables

#### 5. Frontend
- Add Resource > Dockerfile  
- Dockerfile path: `Dockerfile.frontend`
- Port: 3000
- Build args:
  - `BROWSER_BASE_URL=https://api.kweebex.com/v2/`
  - `CF_PAGES_URL=https://kweebex.com`

## DNS Configuration (Cloudflare)

Add these DNS records:

| Type | Name | Content | Proxy |
|------|------|---------|-------|
| A | @ | 161.97.153.92 | Yes |
| A | api | 161.97.153.92 | Yes |
| A | www | 161.97.153.92 | Yes |

### SSL/TLS Settings
1. Go to SSL/TLS > Overview
2. Set encryption mode to "Full (strict)"
3. Enable "Always Use HTTPS"

## HTTPS with Coolify

Coolify handles Let's Encrypt certificates automatically:

1. In your service settings, set the domain (e.g., `api.kweebex.com`)
2. Enable "Generate SSL Certificate"
3. Coolify will automatically provision and renew certificates

## Environment Variables Reference

### Required (API won't start without these)

| Variable | Description |
|----------|-------------|
| `DATABASE_URL` | PostgreSQL connection string |
| `REDIS_URL` | Redis connection string |
| `MEILISEARCH_ADDR` | Meilisearch URL (e.g., `http://meilisearch:7700`) |
| `MEILISEARCH_KEY` | Meilisearch master key |
| `BIND_ADDR` | API bind address (`0.0.0.0:8000`) |
| `SITE_URL` | Public site URL |
| `CDN_URL` | API public URL |
| `SELF_ADDR` | API internal URL |
| `LABRINTH_ADMIN_KEY` | Admin API key |
| `RATE_LIMIT_IGNORE_KEY` | Rate limit bypass key |
| `STORAGE_BACKEND` | `local` or `s3` |
| `MOCK_FILE_PATH` | File storage path (for local backend) |
| `WHITELISTED_MODPACK_DOMAINS` | JSON array of allowed domains |
| `ALLOWED_CALLBACK_URLS` | JSON array of OAuth callback URLs |

### Optional (features disabled if not set)

- OAuth: `GITHUB_CLIENT_ID/SECRET`, `DISCORD_CLIENT_ID/SECRET`, etc.
- Email: `SMTP_*` variables
- Payments: `STRIPE_*`, `PAYPAL_*` variables
- Analytics: `CLICKHOUSE_*` variables

## Troubleshooting

### API returns 404
- Check logs: `docker logs <container>`
- Ensure Meilisearch is running and accessible
- Verify all required env vars are set

### Build fails (out of memory)
- Add swap: `sudo fallocate -l 4G /swapfile && sudo chmod 600 /swapfile && sudo mkswap /swapfile && sudo swapon /swapfile`
- Rust builds need ~4GB RAM

### Database connection fails
- Verify PostgreSQL is healthy: `docker exec <postgres-container> pg_isready`
- Check connection string format

## Architecture

```
                    ┌─────────────────┐
                    │   Cloudflare    │
                    │  (DNS + Proxy)  │
                    └────────┬────────┘
                             │
         ┌───────────────────┼───────────────────┐
         │                   │                   │
         ▼                   ▼                   ▼
  kweebex.com          api.kweebex.com     cdn.kweebex.com
         │                   │                   │
         ▼                   ▼                   │
   ┌──────────┐       ┌──────────┐              │
   │ Frontend │       │ Labrinth │◄─────────────┘
   │  (Nuxt)  │──────▶│  (Rust)  │
   │  :3000   │       │  :8000   │
   └──────────┘       └────┬─────┘
                           │
         ┌─────────────────┼─────────────────┐
         │                 │                 │
         ▼                 ▼                 ▼
   ┌──────────┐     ┌──────────┐     ┌─────────────┐
   │ Postgres │     │  Redis   │     │ Meilisearch │
   │  :5432   │     │  :6379   │     │    :7700    │
   └──────────┘     └──────────┘     └─────────────┘
```
