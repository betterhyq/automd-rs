import { defineConfig } from 'vitepress'

export default defineConfig({
  title: "Automd-RS",
  description: "Update README.md from Cargo.toml, automatically. Badges, contributors, install snippets via HTML comment blocks.",

  themeConfig: {
    nav: [
      { text: 'Guide', link: '/guide/getting-started' },
      { text: 'Block Reference', link: '/guide/block-reference' },
      { text: 'API', link: '/guide/api' },
    ],

    sidebar: [
      {
        text: 'Guide',
        items: [
          { text: 'Getting Started', link: '/guide/getting-started' },
          { text: 'Block Reference', link: '/guide/block-reference' },
          { text: 'API Reference', link: '/guide/api' },
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/betterhyq/automd-rs' }
    ]
  }
})
