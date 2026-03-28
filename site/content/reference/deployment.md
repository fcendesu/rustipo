---
title: Deployment
summary: Understand Rustipo's deployment helpers for GitHub Pages, Cloudflare Pages, and Netlify.
order: 8
---

# Deployment

Rustipo ships deployment workflow helpers for the common static-hosting path.

Current built-in deploy commands are:

- `rustipo deploy github-pages`
- `rustipo deploy cloudflare-pages`
- `rustipo deploy netlify`

These commands generate workflow files. They do not run a remote deploy directly from the CLI.

## GitHub Pages

```bash
rustipo deploy github-pages
```

Current behavior:

- writes `.github/workflows/deploy-pages.yml`
- builds the site with `rustipo build`
- deploys `dist/` with GitHub Pages actions

Good fit when:

- your site already lives on GitHub
- you want the simplest GitHub-native hosting path
- you do not need platform-specific extras from other hosts

## Cloudflare Pages

```bash
rustipo deploy cloudflare-pages
```

Current behavior:

- writes `.github/workflows/deploy-cloudflare-pages.yml`
- builds the site with `rustipo build`
- deploys `dist/` with `cloudflare/wrangler-action`

Required repository settings:

- secret: `CLOUDFLARE_API_TOKEN`
- secret: `CLOUDFLARE_ACCOUNT_ID`
- variable: `CLOUDFLARE_PAGES_PROJECT`

Good fit when:

- you want Cloudflare Pages hosting
- your site already uses Cloudflare infrastructure
- you prefer Pages upload workflows over Git integration

## Netlify

```bash
rustipo deploy netlify
```

Current behavior:

- writes `.github/workflows/deploy-netlify.yml`
- builds the site with `rustipo build`
- deploys `dist/` using Netlify CLI

Required repository settings:

- secret: `NETLIFY_AUTH_TOKEN`
- secret: `NETLIFY_SITE_ID`

Good fit when:

- your site already uses Netlify
- you want a common static-site hosting platform with preview-friendly tooling

## `--force`

Each deploy helper supports `--force` to overwrite an existing generated workflow.

## What These Helpers Do Not Replace

Deployment helpers do not replace:

- choosing a host
- setting platform secrets and variables
- configuring a custom domain
- host-specific cache or preview policies

They are meant to shorten the “generate the right workflow” step.

## Choosing Between Them

Use:

- GitHub Pages for the most GitHub-native path
- Cloudflare Pages when you already want Cloudflare hosting and credentials
- Netlify when your team already prefers Netlify workflows

## Related Guides

- [Deploying Rustipo sites](/guides/deploying-rustipo-sites/)
- [Getting started](/guides/getting-started/)
- [Publishing outputs](/reference/publishing-outputs/)
- [CLI reference](/reference/cli/)
