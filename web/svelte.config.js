import vercel from "@sveltejs/adapter-vercel";
import preprocess from "svelte-preprocess";
// import cloudflare from '@sveltejs/adapter-cloudflare';
/** @type {import('@sveltejs/kit').Config} */
const config = {
  extensions: [".svelte"],
  // Consult https://github.com/sveltejs/svelte-preprocess
  // for more information about preprocessors
  preprocess: [
    preprocess({
      postcss: true,
    }),
  ],
  onwarn: (warning, handler) => {
    const { code, frame } = warning;
    if (code === "css-unused-selector") return;

    handler(warning);
  },

  kit: {
    adapter: vercel(),
    prerender: {
      crawl: true,
      enabled: true,
      onError: "continue",
      entries: ["*"],
    },
    vite: { optimizeDeps: { include: ["lodash.get", "lodash.isequal", "lodash.clonedeep"] } },
  },
};

export default config;
