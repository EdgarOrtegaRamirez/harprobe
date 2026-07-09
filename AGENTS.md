# AGENTS.md — for AI Agents

## Project Overview

HarProbe is a CLI tool for analyzing HTTP Archive (HAR) files. It parses HAR 1.2 JSON format and provides statistics, diffing, filtering, and CI/CD integration.

## Build & Test

```bash
cargo build
cargo test
cargo clippy -- -D warnings
cargo fmt --all -- --check
```

## Project Structure

- `src/main.rs` — CLI entry point (clap)
- `src/lib.rs` — Core library
  - `parse_har()` — Parse HAR JSON
  - `analyze()` — Full statistics
  - `diff_har()` — Compare two HAR files
  - `filter_entries()` — Filter by criteria
  - `check_thresholds()` — CI/CD validation
  - `generate_report()` — Render reports (text, markdown, json)
  - `format_size()` — Human-readable byte sizes
  - `suggest_filename()` — Generate output filenames
  - `extract_domain()` — Extract domain from URL

## Key Types

- `HarFile` — Top-level HAR 1.2 structure
- `HarStats` — Computed analysis results
- `HarDiff` — Diff results between two HAR files
- `EntrySummary` — Summary of a single entry
- `DomainStats` — Per-domain statistics
- `PageLoadSummary` — Page load timing summary

## CI/CD

The GitHub Actions workflow runs on push/PR to main branch:
1. `cargo build`
2. `cargo test`
3. `cargo clippy -- -D warnings`
4. `cargo fmt --check`

## Publishing

```bash
cargo publish
git tag v0.1.0
git push origin v0.1.0
```