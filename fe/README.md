# Frontend

This folder contains the web interface built with [SvelteKit](https://kit.svelte.dev/).

## Prerequisites

- Node.js v18 or higher

## Development

Install dependencies and start the development server:

```bash
cd fe
npm install
npm run dev
```

## Building for Production

```bash
npm run build
```

A preview of the production build can be launched with `npm run preview`.

## Quality Checks

- Format sources with `prettier`.
- Run `npx svelte-check` to perform type and lint checks.
- Execute tests with `npm test` (uses Vitest and Playwright).

Refer to the repository [README](../README.md) for more information about the project as a whole.

## Features

- Login page at `/login` authenticates against the backend `/auth/login` API and stores the returned JWT tokens in `localStorage`.
- The login screen automatically focuses the username field and displays inline error messages when authentication fails.
- After logging in, users are redirected to a dummy dashboard at `/dashboard`.
- Sidebar navigation scrolls independently from the main content for easier access.
- `PageSection` component provides a consistent layout for page sections and is used on the profile page.
