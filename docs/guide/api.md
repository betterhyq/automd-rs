---
title: API Reference
outline: deep
---

# API Reference

## Basic usage

```rust
use automd_rs::run;
use std::path::Path;

fn main() -> automd_rs::Result<()> {
    run(Path::new("."), Path::new("README.md"))?;
    Ok(())
}
```

`run` parses `Cargo.toml`, processes blocks in `README.md`, and writes the updated content.

## Custom handler

Use `run_with_handler` and implement `BlockHandler` for custom block types:

```rust
use automd_rs::{run_with_handler, DefaultHandler};
use std::path::Path;

fn main() -> automd_rs::Result<()> {
    let handler = DefaultHandler::default();
    let updated = run_with_handler(Path::new("."), Path::new("README.md"), &handler)?;
    Ok(())
}
```

## Public API

| Item | Description |
|------|-------------|
| `run` | Run with default handler |
| `run_with_handler` | Run with custom handler |
| `BlockHandler` | Trait for generating block content |
| `DefaultHandler` | Built-in handler |
| `UpdateContext` | Parsed Cargo.toml context |
| `parse_manifest` | → `ParsedManifest` |
| `parse_readme_blocks` | → `BlockRequest` |
| `assign_and_generate` | Generate content per block |
| `replace_blocks_once` | Replace blocks in README |
| `update_readme` | Write updated README |
| `Error`, `Result` | Error types |

## Implementing BlockHandler

```rust
use automd_rs::{BlockHandler, UpdateContext};
use automd_rs::error::Result;

struct MyHandler;

impl BlockHandler for MyHandler {
    fn generate(
        &self,
        block_name: &str,
        open_tag_line: &str,
        context: &UpdateContext,
    ) -> Result<Vec<String>> {
        match block_name {
            "my-block" => Ok(vec!["custom content".into()]),
            _ => Ok(vec![]),
        }
    }
}
```

## docs.rs

Full API docs: [docs.rs/automd-rs](https://docs.rs/automd-rs)
