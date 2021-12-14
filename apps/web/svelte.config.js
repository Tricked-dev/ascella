import vercel from "@sveltejs/adapter-vercel";
import { mdsvex } from "mdsvex";
import preprocess from "svelte-preprocess";
import mdsvexConfig from "./mdsvex.config.js";
// import cloudflare from '@sveltejs/adapter-cloudflare';
/** @type {import('@sveltejs/kit').Config} */
const config = {
    extensions: [".svelte", ...mdsvexConfig.extensions],
    // Consult https://github.com/sveltejs/svelte-preprocess
    // for more information about preprocessors
    preprocess: [
        preprocess({
            postcss: true,
        }),
        mdsvex(mdsvexConfig),
    ],

    kit: {
        adapter: vercel(),
        target: "#main",
        prerender: {
            crawl: true,
            enabled: true,
            onError: "continue",
            entries: ["*"],
        },
        vite: {
            optimizeDeps: {},
        },
    },
};

export default config;
