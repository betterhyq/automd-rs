# automd-rs

<!-- automdrs:badges showCrateVersion="true" showCrateDownloads="true" showCrateDocs="true" showCommitActivity="true" showRepoStars="true" -->
![Crates.io Version](https://img.shields.io/crates/v/automd-rs)
![Crates.io Total Downloads](https://img.shields.io/crates/d/automd-rs)
![docs.rs](https://img.shields.io/docsrs/automd-rs)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/betterhyq/automd-rs)
![GitHub Repo stars](https://img.shields.io/github/stars/betterhyq/automd-rs)
<!-- /automdrs -->

`automd-rs` is a Rust crate that **updates your README.md from your Cargo.toml automatically**. Put special HTML comments in your README; run the tool (CLI or as a library), and it fills those blocks with badges, contributor info, and more—so you don’t have to keep version, repo URL, or author in sync by hand.

### Features

- **Badges** – Crates.io version, downloads, docs.rs, GitHub commit activity, repo stars
- **Contributors** – License line and [contrib.rocks](https://contrib.rocks) image from repo
- **With-automdrs** – Optional “auto updated with automd-rs” footer
- **Cargo-add** – Inserts `cargo add <crate-name>` snippet for adding the crate as a dependency
- **Cargo-install** – Inserts `cargo install <crate-name>` snippet for installing the binary
- **Extensible** – Use the library with a custom `BlockHandler` to add your own blocks

## Installation

**As a dependency** (library):

<!-- automdrs:cargo-add -->

```sh
cargo add automd-rs
```

<!-- /automdrs -->

**As a binary** (to run in your project root):

<!-- automdrs:cargo-install -->

```sh
cargo install automd-rs
```

<!-- /automdrs -->

## Usage

### CLI

From your crate root (where `Cargo.toml` and `README.md` live):

```bash
automd-rs
```

This reads `Cargo.toml`, finds all `<!-- automdrs:... -->` blocks in `README.md`, generates content for each, and overwrites `README.md` with the result.

### Library

```rust
use automd_rs::run;
use std::path::Path;

fn main() -> automd_rs::Result<()> {
    let manifest_dir = Path::new(".");
    let readme_path = Path::new("README.md");
    run(manifest_dir, readme_path)?;
    Ok(())
}
```

For custom block types, use `run_with_handler` and implement the `BlockHandler` trait:

```rust
use automd_rs::{run_with_handler, DefaultHandler, BlockHandler, UpdateContext};
use std::path::Path;

fn main() -> automd_rs::Result<()> {
    let handler = DefaultHandler::default();
    let updated = run_with_handler(Path::new("."), Path::new("README.md"), &handler)?;
    Ok(())
}
```

**Public API:** `run`, `run_with_handler`, `BlockHandler`, `DefaultHandler`, `UpdateContext`, `parse_manifest` (→ `ParsedManifest`), `parse_readme_blocks` (→ `BlockRequest`), `assign_and_generate`, `replace_blocks_once`, `update_readme`, `Error`, `Result`.

### README block syntax

Blocks are HTML comments. Open with `<!-- automdrs:NAME [options] -->`, close with `<!-- /automdrs -->`. The content between is replaced by generated output. Options support both `key="value"` and standalone words (e.g. `version` is treated as `version="true"`).

| Block           | Options (examples) | Description |
|----------------|--------------------|-------------|
| `badges`       | `showCrateVersion`, `showCrateDownloads`, `showCrateDocs`, `showCommitActivity`, `showRepoStars` — or short: `version`, `downloads`, `docs`, `commit_activity`, `repo_stars`. Use `"true"`/`"false"` or standalone word. | Shield badges for crates.io and GitHub |
| `contributors` | `author="YOUR_NAME"`, `license="MIT"` | License line + contrib.rocks image |
| `with-automdrs`| `message="..."` (optional) | Footer: “auto updated with automd-rs” |
| `cargo-add`    | (none)             | Snippet: `cargo add <crate-name>` |
| `cargo-install`| (none)             | Snippet: `cargo install <crate-name>` |

**Example – badges (all on):**

```markdown
<!-- automdrs:badges showCrateVersion="true" showCrateDownloads="true" showCrateDocs="true" showCommitActivity="true" showRepoStars="true" -->
![Crates.io Version](https://img.shields.io/crates/v/automd-rs)
![Crates.io Total Downloads](https://img.shields.io/crates/d/automd-rs)
![docs.rs](https://img.shields.io/docsrs/automd-rs)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/betterhyq/automd-rs)
![GitHub Repo stars](https://img.shields.io/github/stars/betterhyq/automd-rs)
<!-- /automdrs -->
```

**Example – contributors:**

```markdown
<!-- automdrs:contributors author="YONGQI" license="MIT" -->
Published under the [MIT](./LICENSE) license.
Made by [@YONGQI](https://github.com/betterhyq) 💛
<br><br>
<a href="https://github.com/betterhyq/automd-rs/graphs/contributors">
<img src="https://contrib.rocks/image?repo=betterhyq/automd-rs" />
</a>
<!-- /automdrs -->
```

**Example – cargo-add / cargo-install:** (used in the Installation section above)

```markdown
<!-- automdrs:cargo-add -->

```sh
cargo add automd-rs
```

<!-- /automdrs -->

<!-- automdrs:cargo-install -->

```sh
cargo install automd-rs
```

<!-- /automdrs -->
```

Your `Cargo.toml` must include `repository = "https://github.com/owner/repo"` (or similar) so the tool can derive the GitHub owner/repo for badges and contributors.

## Contribution

<details>
  <summary>Local development</summary>

- Clone this repository
- Install the latest version of [Rust](https://rust-lang.org/)
- Run tests using `cargo test` or `cargo run`

</details>

## Credits

`automd-rs` was inspired by the idea of keeping README and crate metadata in sync via comment-driven blocks, similar in spirit to doc-generation and badge tools in the Rust and JavaScript ecosystems, such as [@automd](https://automd.unjs.io).

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