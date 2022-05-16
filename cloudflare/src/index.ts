export default {
  async fetch(request: Request) {
    try {
      return await handleRequest(request);
    } catch (e) {
      return new Response(`${e}`);
    }
  },
};
const headers = ["discord", "github", "twitter", "youtube", "instagram", "linkedin", "github", "twitter", "youtube", "instagram", "linkedin", "element", "revolt", "curl", "matrix", "cinny", "reddit"];

async function handleRequest(req: Request) {
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
      const rson = await r.json() as any;
      if (rson.redirect) {
        return Response.redirect(rson.redirect);
      }
      // TODO(tricked): add this back when cloudflare images are being used!
      // if (!rson.embed.title && !rson.embed.description) {
      //   let res = await fetch(`https://ascella.wtf/v2/ascella/view/${name}.png`);
      //   return new Response(res.body, {
      //     headers: {
      //       "content-type": rson.content_type,
      //     },
      //   });
      // }

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
}
