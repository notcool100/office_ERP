# Frontend Dockerfile for Coolify
# Builds the SvelteKit Node app and serves it on port 3110.
FROM node:20-alpine AS builder
WORKDIR /app

RUN npm install -g pnpm
COPY package.json pnpm-lock.yaml svelte.config.js tsconfig.json ./
RUN pnpm install
COPY . .
RUN pnpm run build

FROM node:20-alpine AS runner
WORKDIR /app
RUN npm install -g pnpm
COPY --from=builder /app/build ./build
COPY package.json pnpm-lock.yaml ./
RUN pnpm install --prod

EXPOSE 3110
ENV PORT=3110
CMD ["node", "build/index.js"]
