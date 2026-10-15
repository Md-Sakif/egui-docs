// @ts-check
import {themes as prismThemes} from 'prism-react-renderer';

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: 'egui & eframe',
  tagline: 'Build desktop and web apps in Rust, one friendly step at a time',
  favicon: 'img/logo.svg',

  // Where the site is published: https://md-sakif.github.io/egui-docs/
  // If you rename the GitHub repository, change baseUrl to match.
  url: 'https://md-sakif.github.io',
  baseUrl: '/egui-docs/',
  organizationName: 'Md-Sakif',
  projectName: 'egui-docs',
  trailingSlash: false,

  onBrokenLinks: 'throw',
  markdown: {
    hooks: {
      onBrokenMarkdownLinks: 'throw',
    },
  },

  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  presets: [
    [
      'classic',
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          sidebarPath: './sidebars.js',
        },
        blog: false,
        theme: {
          customCss: './src/css/custom.css',
        },
      }),
    ],
  ],

  themes: [
    [
      '@easyops-cn/docusaurus-search-local',
      /** @type {import('@easyops-cn/docusaurus-search-local').PluginOptions} */
      ({
        hashed: true,
        indexBlog: false,
        docsRouteBasePath: '/docs',
      }),
    ],
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      colorMode: {
        respectPrefersColorScheme: true,
      },
      navbar: {
        title: 'egui & eframe',
        logo: {
          alt: 'egui docs logo',
          src: 'img/logo.svg',
        },
        items: [
          {type: 'docSidebar', sidebarId: 'docs', position: 'left', label: 'Docs'},
          {to: '/docs/widgets', label: 'Widgets', position: 'left'},
          {to: '/docs/containers', label: 'Containers', position: 'left'},
          {to: '/docs/cookbook', label: 'Cookbook', position: 'left'},
          {href: 'https://docs.rs/egui', label: 'API reference', position: 'right'},
          {href: 'https://github.com/emilk/egui', label: 'GitHub', position: 'right'},
        ],
      },
      footer: {
        style: 'dark',
        links: [
          {
            title: 'Docs',
            items: [
              {label: 'Introduction', to: '/docs/'},
              {label: 'Getting started', to: '/docs/getting-started/installation'},
              {label: 'Widgets', to: '/docs/widgets'},
            ],
          },
          {
            title: 'egui',
            items: [
              {label: 'egui on GitHub', href: 'https://github.com/emilk/egui'},
              {label: 'Online demo', href: 'https://www.egui.rs'},
              {label: 'API reference (docs.rs)', href: 'https://docs.rs/egui'},
            ],
          },
          {
            title: 'Community',
            items: [
              {label: 'Discussions', href: 'https://github.com/emilk/egui/discussions'},
              {label: 'Discord', href: 'https://discord.gg/JFcEma9bJq'},
            ],
          },
        ],
        copyright:
          'Unofficial documentation for egui and eframe 0.36. Every example on this site is compiled and tested.',
      },
      prism: {
        theme: prismThemes.github,
        darkTheme: prismThemes.dracula,
        additionalLanguages: ['rust', 'toml', 'bash', 'powershell'],
      },
    }),
};

export default config;
