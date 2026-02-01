# just-repo

Short monorepo with a Next.js frontend and a Rust API. Turborepo and pnpm keep the workspace fast and consistent.

### Usage
```sh
pnpm install
pnpm dev
```

### Monorepo tooling
- Turborepo for task orchestration (`turbo dev`)
- pnpm workspaces for package management

### Frontend (Next.js) - `apps/next`
- Next.js 16 (App Router) + React 19 + TypeScript
- Tailwind CSS v4 (via PostCSS)
- Biome for linting/formatting
- Env + validation: `@t3-oss/env-nextjs` + Zod

### Backend (Rust) - `apps/rust`
- Rust 2024 edition + Cargo
- Axum + Tokio for the web server runtime
- Serde/serde_json for JSON handling
- Vercel runtime + tower-http (CORS)
