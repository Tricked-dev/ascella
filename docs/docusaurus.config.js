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
      "classic",
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          sidebarPath: require.resolve("./sidebars.js"),
          // Please change this to your repo.
          editUrl: "https://github.com/Tricked-dev/ascella",
        },
        // blog: {
        //   showReadingTime: true,
        //   // Please change this to your repo.
        //   editUrl:
        //     'https://github.com/facebook/docusaurus/tree/main/packages/create-docusaurus/templates/shared/',
        // },
        theme: {
          customCss: require.resolve("./src/css/custom.css"),
        },
      }),
    ],
    [
      "redocusaurus",
      {
        debug: Boolean(process.env.DEBUG || process.env.CI),
        specs: [
          {
            // spec: '/openapi.json',
            spec: "https://converter.swagger.io/api/convert?url=https://ascella.wtf/v2/ascella/spec/v2",
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
                to: "/docs/",
              },
            ],
          },

          {
            title: "Styff",
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
                href: "https://github.com/Tricked-dev/ascellas",
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
