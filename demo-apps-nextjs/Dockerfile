FROM node:20-slim AS builder

WORKDIR /app

COPY package*.json ./
RUN npm install

COPY . .
RUN npm run build

# Production image — only the standalone output is copied, so secrets
# mounted into /app/secrets at runtime (see docker-compose.demo.yml) never
# pass through the build stage.
FROM node:20-slim AS runner

WORKDIR /app
ENV NODE_ENV=production
# Required for standalone output: without it the server binds to "localhost"
# only, so the container's own loopback (used by the healthcheck) can't
# reach it even though externally-published traffic still gets through.
ENV HOSTNAME="0.0.0.0"

COPY --from=builder /app/public ./public
COPY --from=builder /app/.next/standalone ./
COPY --from=builder /app/.next/static ./.next/static

EXPOSE 3000
CMD ["node", "server.js"]
