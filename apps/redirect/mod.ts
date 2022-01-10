import { listenAndServe, serve } from "https://deno.land/std@0.117.0/http/server.ts";

import {
    AugmentedRequest,
    createRouteMap,
    createRouter,
    jsonResponse,
    MissingRouteError,
    streamResponse,
} from "https://deno.land/x/reno@v2.0.17/reno/mod.ts";

/* Alternatively, you can import Reno from nest.land:
 * import { ... } from "https://x.nest.land/reno@2.0.17/reno/mod.ts";
 */

function createErrorResponse(status: number, { message }: Error) {
    return new Response(message, {
        status,
    });
}

export const routes = createRouteMap([
    [
        /(.*)/,
        async (req: AugmentedRequest) => {
            let id = req.pathname.split("/");
            let name = id[id.length - 1];
            if (name.includes(".")) {
                name = name.split(".")[0];
            }
            if (req.headers.get("user-agent")?.toLowerCase().includes("discord")) {
                let r = await fetch(
                    `https://ascella.wtf/v2/ascella/view/${name}/stats`,
                );
                if (r.ok) {
                    let rson = await r.json();

                    // {"content_type":"image/png","embed":{"color":"","description":"","title":"","url":""},"id":2814,"image_size":"1.05 KB","redirect":null,"user_id":0,"user_name":"tricked"}
                    return new Response(
                        `<html>
<head>
  <title>${rson.user_name} | Ascella.host</title>
  <meta charset="utf-8">
  <meta name="robots" content="noindex">
  <meta name="theme-color" content="${rson.embed?.color}">
  <meta property="og:title" content="${rson.embed?.title}">
  <meta property="og:description" content="${rson.embed?.description}">
  <meta property="og:image" content="${`https://ascella.wtf/v2/ascella/view/${name}`}">
  <meta property="twitter:card" content="summary_large_image">
</head>
<body>
  <img src="${`https://ascella.wtf/v2/ascella/view/${name}`}">
</body>
</html>`,
                        {
                            headers: {
                                "content-type": "text/html; charset=UTF-8",
                            },
                        },
                    );
                } else {
                    return await new Response(undefined, {
                        status: 301,
                        headers: new Headers({
                            ["location"]: `https://ascella.host/${name}`,
                        }),
                    });
                }
            } else {
                return await new Response(undefined, {
                    status: 301,
                    headers: new Headers({
                        ["location"]: `https://ascella.host/${name}`,
                    }),
                });
            }
        },
    ],
]);

const notFound = (e: MissingRouteError) => createErrorResponse(404, e);
const serverError = (e: Error) => createErrorResponse(500, e);

const mapToErrorResponse = (e: Error) => e instanceof MissingRouteError ? notFound(e) : serverError(e);

const router = createRouter(routes);

console.log("Listening for requests...");

await serve(
    async (req) => {
        try {
            return await router(req);
        } catch (e) {
            return mapToErrorResponse(e);
        }
    },
    { addr: ":3000" },
);
