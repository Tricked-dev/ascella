// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const lightCodeTheme = require("prism-react-renderer/themes/github");
const darkCodeTheme = require("prism-react-renderer/themes/dracula");
/** @type {import('@docusaurus/types').Config} */
const config = {
  title: "Ascella docs",
  tagline: "Ascella is the fastest opensource image uploader",
  url: "https://docs.ascella.host",
  baseUrl: "/",
  onBrokenLinks: "throw",
  onBrokenMarkdownLinks: "warn",
  favicon: "img/favicon.ico",
  organizationName: "tricked-dev", // Usually your GitHub org/user name.
  projectName: "ascella", // Usually your repo name.

  presets: [
    [
      "@docusaurus/preset-classic",
      {
        docs: {
          routeBasePath: "/", // Serve the docs at the site's root
          /* other docs plugin options */
        },
        blog: false, // Optional: disable the blog plugin
        // ...
      },
    ],
    [
      "redocusaurus",
      {
        debug: Boolean(process.env.DEBUG || process.env.CI),
        specs: [
          {
            spec: "https://ascella.wtf/v2/ascella/spec/v3",
            route: "/api/",
          },
        ],
        theme: {
          primaryColor: "#1890ff",
          redocOptions: { hideDownloadButton: false },
        },
      },
    ],
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      colorMode: {
        defaultMode: "dark",
      },
      navbar: {
        title: "Ascella",
        logo: {
          alt: "Ascella Logo",
          src: "img/logo.svg",
        },
        items: [
          {
            type: "doc",
            docId: "index",
            position: "left",
            label: "Docs",
          },
          {
            href: "/api",
            position: "left",
            label: "API",
          },
          {
            href: "https://github.com/Tricked-dev/ascella",
            label: "GitHub",
            position: "right",
          },
        ],
      },
      footer: {
        style: "dark",
        links: [
          {
            title: "Docs",
            items: [
              {
                label: "Getting started",
                to: "/",
              },
            ],
          },

          {
            title: "Stuff",
            items: [
              {
                label: "Terms",
                to: "/rules",
              },
              {
                label: "Privacy",
                to: "/privacy",
              },
            ],
          },
          {
            title: "Community",
            items: [
              {
                label: "Discord",
                href: "https://discord.com/invite/mY8zTARu4g",
              },
            ],
          },
          {
            title: "More",
            items: [
              {
                label: "GitHub",
                href: "https://github.com/Tricked-dev/ascella",
              },
            ],
          },
        ],
        copyright: `Copyright © ${new Date().getFullYear()} Ascella!, Inc. Built with DeezNuts.`,
      },
      prism: {
        theme: lightCodeTheme,
        darkTheme: darkCodeTheme,
      },
    }),
};

module.exports = config;
