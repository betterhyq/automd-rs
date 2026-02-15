---
title: Block Reference
outline: deep
---

# Block Reference

Blocks use HTML comments. Open with `<!-- automdrs:NAME [options] -->`, close with `<!-- /automdrs -->`. Content between is replaced by generated output.

Options support `key="value"` and standalone words (e.g. `version` = `version="true"`).

## `badges`

Shield badges for crates.io and GitHub.

| Option | Short | Description |
|--------|-------|-------------|
| `showCrateVersion` | `version` | Crates.io version badge |
| `showCrateDownloads` | `downloads` | Total downloads |
| `showCrateDocs` | `docs` | docs.rs badge |
| `showCommitActivity` | `commit_activity` | GitHub commit activity |
| `showRepoStars` | `repo_stars` | GitHub stars |

**Example (all on):**

```markdown
<!-- automdrs:badges showCrateVersion="true" showCrateDownloads="true" showCrateDocs="true" showCommitActivity="true" showRepoStars="true" -->
<!-- /automdrs -->
```

**Short form:**

```markdown
<!-- automdrs:badges version downloads docs commit_activity repo_stars -->
<!-- /automdrs -->
```

## `contributors`

License line and contrib.rocks image.

| Option | Description |
|--------|-------------|
| `author` | Author name for credit line |
| `license` | License name (e.g. MIT) |

**Example:**

```markdown
<!-- automdrs:contributors author="YONGQI" license="MIT" -->
<!-- /automdrs -->
```

## `with-automdrs`

Footer: "auto updated with automd-rs".

| Option | Description |
|--------|-------------|
| `message` | Custom message (optional) |

**Example:**

```markdown
<!-- automdrs:with-automdrs -->
<!-- /automdrs -->
```

## `cargo-add`

Inserts `cargo add <crate-name>` snippet.

```markdown
<!-- automdrs:cargo-add -->
<!-- /automdrs -->
```

## `cargo-install`

Inserts `cargo install <crate-name>` snippet.

```markdown
<!-- automdrs:cargo-install -->
<!-- /automdrs -->
```
