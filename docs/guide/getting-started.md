---
title: Getting Started
outline: deep
---

# Getting Started

## Prerequisites

- Rust toolchain
- Your crate has `repository = "https://github.com/owner/repo"` in `Cargo.toml`

## Installation

**As a dependency** (library):

```sh
cargo add automd-rs
```

**As a binary** (CLI):

```sh
cargo install automd-rs
```

## Usage

From your crate root (where `Cargo.toml` and `README.md` live):

```bash
automd-rs
```

The tool reads `Cargo.toml`, finds all `<!-- automdrs:... -->` blocks in `README.md`, generates content for each, and overwrites the file.

## First block

Add a badges block to your README:

```markdown
<!-- automdrs:badges version downloads docs -->
<!-- /automdrs -->
```

Run `automd-rs` and the block will be filled with shield badges.

## Next steps

- [Block Reference](/guide/block-reference) — All block types and options
- [API Reference](/guide/api) — Library usage and custom handlers
