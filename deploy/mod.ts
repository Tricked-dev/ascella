/**
 * Deno deply embed service
 * fetches the backend to check if a image exists and sends some plain,
 * for various services to be able to embed the image as intended
 *
 * We use deno deploy since its faster than sveltekit dynamic routes!
 */

import { serve } from "https://deno.land/std@0.117.0/http/server.ts";

console.log("Listening for requests... PORT: 3000");
const headers = ["discord", "github", "twitter", "youtube", "instagram", "linkedin", "github", "twitter", "youtube", "instagram", "linkedin", "element", "revolt", "curl"];
await serve(
  async (req) => {
    const name = new URL(req.url).pathname.split("/").at(-1) || "";
    if (name.includes(".")) {
      const [id, ext] = name.split(".");
      return Response.redirect(`https://ascella.wtf/v2/ascella/view/${id}.${ext}`);
    }
    if (headers.some(x => req.headers.get("user-agent")?.toLowerCase().includes(x))) {
      const r = await fetch(
        `https://ascella.wtf/v2/ascella/view/${name}/stats`,
      );

      if (r.ok) {
        const rson = await r.json();

        // {"content_type":"image/png","embed":{"color":"","description":"","title":"","url":""},"id":2814,"image_size":"1.05 KB","redirect":null,"user_id":0,"user_name":"tricked"}
        return new Response(
          [
            `<html>`,
            `<head>`,
            `<title>${rson.user_name} | Ascella.host</title>`,
            `<meta charset="utf-8">`,
            `<meta name="robots" content="noindex">`,
            rson.embed?.color ? `<meta name="theme-color" content="${rson.embed?.color}">` : "",
            rson.embed?.title ? `<meta property="og:title" content="${rson.embed?.title}">` : "",
            rson.embed?.description ? `<meta property="og:description" content="${rson.embed?.description}">` : "",
            `<meta property="og:image" content="${`https://ascella.wtf/v2/ascella/view/${name}.png`}">`,
            `<meta property="twitter:card" content="summary_large_image">`,
            `</head>`,
            `<body>`,
            `<img src="${`https://ascella.wtf/v2/ascella/view/${name}.png`}">`,
            `</body>`,
            `</html>`,
          ].join(""),
          {
            headers: {
              "content-type": "text/html; charset=UTF-8",
            },
          },
        );
      }
    }
    return Response.redirect(`https://ascella.host/${name}`, 301);
  },
  { addr: ":3000" },
);
