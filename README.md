# automd-rs

<!-- automdrs:badges showCrateVersion="true" showCrateDownloads="true" showCrateDocs="true" showCommitActivity="true" showRepoStars="true" -->
![Crates.io Version](https://img.shields.io/crates/v/automd-rs)
![Crates.io Total Downloads](https://img.shields.io/crates/d/automd-rs)
![docs.rs](https://img.shields.io/docsrs/automd-rs)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/betterhyq/automd-rs)
![GitHub Repo stars](https://img.shields.io/github/stars/betterhyq/automd-rs)
<!-- /automdrs -->

Keep your README.md in sync with Cargo.toml—badges, contributors, install snippets—via HTML comment blocks.

**[Full documentation →](https://betterhyq.github.io/automd-rs/)**

## Quick start

```sh
cargo add automd-rs    # as dependency
# or
cargo install automd-rs   # as CLI
```

```bash
automd-rs   # run in crate root
```

Add blocks in README.md, e.g.:

```markdown
<!-- automdrs:badges version downloads docs -->
![Crates.io Version](https://img.shields.io/crates/v/automd-rs)
![Crates.io Total Downloads](https://img.shields.io/crates/d/automd-rs)
![docs.rs](https://img.shields.io/docsrs/automd-rs)
<!-- /automdrs -->
```

Requires `repository = "https://github.com/owner/repo"` in Cargo.toml.

## Block types

| Block | Purpose |
|-------|---------|
| `badges` | Crates.io version, downloads, docs.rs, GitHub stats |
| `contributors` | License + contrib.rocks image |
| `with-automdrs` | Footer line |
| `cargo-add` / `cargo-install` | Add/install snippet |

See [Block Reference](https://betterhyq.github.io/automd-rs/guide/block-reference) for options.

## Library

```rust
use automd_rs::run;
run(Path::new("."), Path::new("README.md"))?;
```

Extend via `BlockHandler` trait. See [API Reference](https://betterhyq.github.io/automd-rs/guide/api).

## License

<!-- automdrs:contributors author="YONGQI" license="MIT" -->
Published under the [MIT](./LICENSE) license.
Made by [@YONGQI](https://github.com/betterhyq) 💛
<br><br>
<a href="https://github.com/betterhyq/automd-rs/graphs/contributors">
<img src="https://contrib.rocks/image?repo=betterhyq/automd-rs" />
</a>
<!-- /automdrs -->

<!-- automdrs:with-automdrs -->

---

_🛠️ auto updated with [automd-rs](https://github.com/betterhyq/automd-rs)_

<!-- /automdrs -->