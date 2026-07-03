# Foreman Reports

Web application for visualizing infrastructure reports extracted from Foreman. It provides dashboards with charts and tables about the state of managed hosts: operating systems, hardware, erratas, update compliance, domains, and inventory data.

## Architecture

- **Backend**: REST API in Rust (Axum) with LDAP authentication, OpenAPI documentation (Swagger UI), and static file serving.
- **Frontend**: SPA built with SolidJS, using Chart.js for charts (doughnut, line, bar) and TanStack Table for tables with filters and pagination.
- **Database**: PostgreSQL with materialized views that periodically aggregate data from Foreman via cron.

## Project Structure

```
backend/          Rust server (Axum)
frontend/         SolidJS SPA + Vite
postgresql/       SQL scripts (DB creation, materialized views, cron)
axinecgrafana1/   Deployment configuration
DESIGN.md         Design system (tokens, components, themes)
```

## Requirements

- Rust (edition 2021)
- Node.js + pnpm
- PostgreSQL
- LDAP server for authentication

## Development

### Backend

```bash
cd backend
cargo run
```

The server starts on `127.0.0.1:3000` by default (configurable in `configuration.toml`).

### Frontend

```bash
cd frontend
pnpm install
pnpm dev
```

### Database

1. Create the database and user with `postgresql/create.sql`.
2. Create the materialized views with `postgresql/mv.sql` and `postgresql/mv_openscap.sql`.
3. Set up the refresh cron job (`postgresql/foreman_reports.cron`).

## Configuration

The backend is configured via `backend/configuration.toml`:

- **server**: listen address, document root, log level, concurrency limit.
- **database**: materialized view TTL, max connections.
- **ldap**: server URL, domain, search base, required group membership.

## Dashboard Sections

- **Home**: general overview (environment, OS, hardware, updates)
- **Updated**: update status by environment, location, and admin group
- **Erratas**: pending erratas per host
- **Hardware**: hardware distribution and historical trends
- **Domains**: domains and their evolution over time
- **Facts**: free-form query of Foreman facts
- **Inventory**: detailed host inventory with resources
