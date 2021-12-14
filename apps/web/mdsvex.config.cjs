module.exports = {
    extensions: [".svelte.md", ".md", ".svx"],
    smartypants: {
        dashes: "oldschool",
    },
    remarkPlugins: [
        [
            require("remark-github"),
            {
                // Use your own repository
                repository: "https://github.com/tricked-dev/images-frontend",
            },
        ],
        require("remark-abbr"),
    ],
    rehypePlugins: [
        require("rehype-slug"),
        [
            require("rehype-autolink-headings"),
            {
                behavior: "wrap",
            },
        ],
    ],
    layout: {
        blog: "./src/lib/BlogLayout.svelte",
    },
};
