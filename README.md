# just-repo

Short monorepo with a Next.js frontend and a Rust API. Turborepo and pnpm keep the workspace fast and consistent.

### Usage
```sh
pnpm install
pnpm dev
```

### Deployment
Designed to run on **Vercel** for both the frontend and the Rust API. You can deploy elsewhere, but you’ll need to configure the environment and runtime yourself.

### Environment variables
- `NEXT_PUBLIC_API_URL` is required in production (Vercel) so the homepage can fetch the API response. The prod build **will fail** without it, since `@t3-oss/env-nextjs` validates env vars at build time.

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
