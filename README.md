# Kweebex

**The open source modding platform for Hytale**

> Fork of [Modrinth](https://github.com/modrinth/code) adapted for Hytale

## What is Kweebex?

Kweebex is a mod hosting and discovery platform for Hytale, built on the excellent Modrinth codebase. It provides:

- **Web Interface** - Browse, search, and download Hytale mods
- **Desktop App** - Manage your Hytale mods and profiles
- **API** - For mod developers and third-party tools

## Project Structure

```
apps/
  frontend/       # Nuxt 3 web interface
  app/            # Tauri desktop application
  app-frontend/   # Desktop app Vue frontend
  labrinth/       # Actix-web API backend
  docs/           # Documentation site

packages/
  ui/             # Shared Vue components
  assets/         # Branding, icons, styles
  daedalus/       # Game version management (Rust)
  app-lib/        # Desktop app logic (Rust)
  api-client/     # TypeScript API client

config/
  brand.ts        # Branding configuration
  game.ts         # Hytale game configuration
```

## Development

### Prerequisites

- Node.js 20+
- pnpm 9+
- Rust 1.90+

### Quick Start

```bash
pnpm install
pnpm web:dev      # Start web frontend
pnpm app:dev      # Start desktop app
```

## Upstream Sync

This project tracks upstream Modrinth for bug fixes and improvements:

```bash
git fetch upstream
git merge upstream/main
```

## Attribution

Based on [Modrinth](https://modrinth.com) - the open source modding platform for Minecraft.

## License

See [COPYING.md](COPYING.md) for license information.
