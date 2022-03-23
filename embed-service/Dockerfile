FROM denoland/deno:latest as base

WORKDIR /app

COPY . ./

RUN deno cache deploy.ts

CMD ["run", "--allow-net","--allow-env", "deploy.ts"]