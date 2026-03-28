---
title: Deploying Rustipo Sites
summary: Choose between GitHub Pages, Cloudflare Pages, and Netlify, then generate the right workflow for your Rustipo site.
order: 4
---

# Deploying Rustipo Sites

Rustipo builds static output. Deployment is the step where you decide where that output should live and how it should be published.

Rustipo's built-in deploy helpers do not push directly to a host from the CLI. Instead, they generate workflow files that build `dist/` and hand deployment off to the chosen platform.

That means the normal shape is:

1. build and validate your site locally
2. choose a host
3. generate the matching workflow with `rustipo deploy ...`
4. add the required repository variables or secrets
5. let your hosting workflow publish `dist/`

## Before You Deploy

Make sure the local publishing loop is healthy first:

```bash
rustipo check
rustipo build
```

That helps you separate content or routing problems from hosting problems.

## The Three Supported Paths

Rustipo currently ships deploy helpers for:

- GitHub Pages
- Cloudflare Pages
- Netlify

All three are good fits for a Rustipo site. The right choice depends more on your hosting environment and team habits than on the generator itself.

## GitHub Pages

Generate the workflow with:

```bash
rustipo deploy github-pages
```

Rustipo writes:

- `.github/workflows/deploy-pages.yml`

The generated workflow:

- installs the published `rustipo` binary
- runs `rustipo build`
- deploys `dist/` with GitHub Pages actions

### Choose GitHub Pages when

- your site already lives on GitHub
- you want the most GitHub-native deployment path
- you want the smallest amount of extra platform setup

### Tradeoffs

GitHub Pages is the simplest path, but it is also the most GitHub-specific. If you already want Cloudflare or Netlify features, it may not be the best long-term host just because it is easy to generate first.

## Cloudflare Pages

Generate the workflow with:

```bash
rustipo deploy cloudflare-pages
```

Rustipo writes:

- `.github/workflows/deploy-cloudflare-pages.yml`

The generated workflow:

- installs the published `rustipo` binary
- runs `rustipo build`
- deploys `dist/` with Wrangler

### Required repository settings

- secret: `CLOUDFLARE_API_TOKEN`
- secret: `CLOUDFLARE_ACCOUNT_ID`
- variable: `CLOUDFLARE_PAGES_PROJECT`

### Choose Cloudflare Pages when

- you already use Cloudflare infrastructure
- you want Cloudflare Pages as the host
- you prefer upload-based deployment from GitHub Actions instead of host-managed Git integration

### Tradeoffs

Cloudflare Pages asks for more platform-specific setup than GitHub Pages, so it is a stronger fit for teams that already know they want Cloudflare.

## Netlify

Generate the workflow with:

```bash
rustipo deploy netlify
```

Rustipo writes:

- `.github/workflows/deploy-netlify.yml`

The generated workflow:

- installs the published `rustipo` binary
- runs `rustipo build`
- deploys `dist/` with Netlify CLI

### Required repository settings

- secret: `NETLIFY_AUTH_TOKEN`
- secret: `NETLIFY_SITE_ID`

### Choose Netlify when

- your site already uses Netlify
- your team already prefers Netlify workflows
- you want a familiar static-hosting setup with preview-oriented tooling

### Tradeoffs

Netlify is a good general-purpose static host, but it is not automatically the best default if your site already lives naturally in a GitHub Pages or Cloudflare workflow.

## A Quick Choice Rule

Use this rule if you want to decide fast:

- choose GitHub Pages when you want the simplest GitHub-native path
- choose Cloudflare Pages when Cloudflare is already part of your stack
- choose Netlify when your team already prefers Netlify as the host

## What Rustipo Generates Versus What You Still Own

Rustipo helps with the workflow file generation step.

Rustipo does not replace:

- choosing the host itself
- adding repository secrets or variables
- configuring a custom domain
- host-specific cache, preview, or branch policies
- platform-specific DNS setup

So the deploy helpers shorten setup, but they do not remove normal hosting decisions.

## `--force`

Each deploy helper supports `--force` if you want to overwrite an existing generated workflow file.

Example:

```bash
rustipo deploy netlify --force
```

## A Healthy Rustipo Deploy Workflow

For most teams, this sequence is enough:

1. write and preview locally with `rustipo dev`
2. validate with `rustipo check`
3. build with `rustipo build`
4. generate one deploy workflow with the host you actually want
5. configure the required repository settings once
6. let CI handle future publishes

## Common Mistakes

The most common deployment mistakes are:

- choosing a host before the local build is healthy
- treating deploy helpers as if they publish directly from the CLI
- forgetting required repository variables or secrets
- generating multiple deploy workflows without deciding which host is the real one

## Good Companion Pages

- [Getting started](/guides/getting-started/)
- [Building the docs site](/guides/building-the-docs-site/)
- [CLI reference](/reference/cli/)
- [Deployment](/reference/deployment/)
