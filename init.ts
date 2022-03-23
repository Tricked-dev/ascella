import Ask from "https://deno.land/x/ask/mod.ts";

const ask = new Ask({
  prefix: "[?]",
});

const answers = await ask.prompt([
  {
    name: "POSTGRES_USER",
    message: "Postgres User:",
    type: "input",
  },
  {
    name: "POSTGRES_PASSWORD",
    message: "Postgres Password:",
    type: "input",
  },
  {
    name: "DISCORD_TOKEN",
    message: "Discord Token(for bot):",
    type: "input",
  },
  {
    name: "APPLICATION_ID",
    message: "Discord Bot Id(for bot):",
    type: "input",
  },
  {
    name: "WEBHOOK",
    message: "Webhook for logs:",
    type: "input",
  },
  {
    name: "S3_ID",
    message: "S3 ID:",
    type: "input",
  },
  {
    name: "S3_SECRET",
    message: "S3 Secret:",
    type: "input",
  },
  {
    name: "S3_BUCKET",
    message: "S3 Bucket:",
    type: "input",
  },
  {
    name: "BACKEND_URL",
    message: "Backend Url:",
    type: "input",
  },
  {
    name: "FRONTEND_URL",
    message: "Frontend Url:",
    type: "input",
  },
  {
    name: "DASHBOARD_URL",
    message: "Dashboard Url:",
    type: "input",
  },
]);
answers.POSTGRES_DB = "ascella";
answers.DATABASE_URL =
  `postgres://${answers.POSTGRES_USER}:${answers.POSTGRES_PASSWORD}@localhost:5432/ascella`;
Deno.writeTextFile(
  ".env",
  Object.entries(answers).map((x) => `${x[0]}=${x[1]}`).join("\n"),
);
console.log("[!] Ascella config successfully written");
