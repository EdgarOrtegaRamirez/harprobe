# HarProbe

**HTTP Archive (HAR) File Analysis CLI**

HarProbe is a Rust CLI tool for analyzing, comparing, and reporting on HTTP Archive (HAR) files. It provides detailed statistics, performance insights, diff capabilities, and CI/CD integration — all from the command line.

## Features

- **Statistics**: Total requests, pages, transfer size, timing, status code distribution, content types, methods, cache analysis
- **Performance**: Identify slowest requests, domain-level timing breakdown, page load events
- **Diff**: Compare two HAR files to detect added/removed requests and performance regressions
- **Filtering**: Filter entries by status code range, URL pattern, content type, and timing thresholds
- **Reporting**: Plain text, Markdown, and JSON output formats
- **CI/CD mode**: Exit with non-zero if configurable thresholds are exceeded
- **No external service dependencies**: Runs entirely offline

## Installation

### From source

```bash
git clone https://github.com/EdgarOrtegaRamirez/harprobe.git
cd harprobe
cargo build --release
# Binary at ./target/release/harprobe
```

### Using cargo

```bash
cargo install --git https://github.com/EdgarOrtegaRamirez/harprobe.git
```

## Usage

### Basic analysis

```bash
# Analyze a HAR file
harprobe analyse sample.har

# Generate a Markdown report
harprobe analyse sample.har --format markdown -o report.md

# Generate JSON output
harprobe analyse sample.har --format json -o output.json
```

### Diff two HAR files

```bash
# Compare two HAR files
harprobe diff before.har after.har
```

### Filter entries

```bash
# Filter by status codes 4xx-5xx
harprobe filter sample.har --status-min 400

# Filter by URL pattern
harprobe filter sample.har --url-pattern api

# Filter by minimum time
harprobe filter sample.har --min-time 200
```

### CI/CD mode

```bash
# Fail if total time > 5000ms, requests > 50, or errors > 5
harprobe check sample.har --max-total-time 5000 --max-requests 50 --max-errors 5
```

## Commands

| Command | Description |
|---------|-------------|
| `analyse` | Full HAR analysis with stats, slowest requests, domain breakdown |
| `diff` | Compare two HAR files side by side |
| `filter` | Filter entries by status, URL, content type, or timing |
| `check` | Validate against CI/CD thresholds |

### Output formats

| Format | Description |
|--------|-------------|
| `text` (default) | Terminal-friendly plain text with Unicode charts |
| `markdown` | Markdown report suitable for CI artifacts |
| `json` | Machine-readable JSON for scripting |

## HAR File Format

HarProbe supports the [HAR 1.2 specification](http://www.softwareishard.com/blog/har-12-spec/) — the standard format for HTTP request/response archives exported by browsers (Chrome DevTools, Firefox) and API testing tools.

## Architecture

```
src/
├── main.rs     # CLI entry point (clap-based argument parsing)
├── lib.rs      # Core library: parsing, analysis, diff, filtering, reporting
└── tests/      # Unit and integration tests
```

The library API (`lib.rs`) exposes these public functions:

- `parse_har(bytes: &[u8]) -> Result<HarFile>` — Parse HAR JSON
- `analyze(har: &HarFile) -> HarStats` — Compute full statistics
- `diff_har(before: &HarFile, after: &HarFile) -> HarDiff` — Compare two HAR files
- `filter_entries(...)` — Filter entries by criteria
- `check_thresholds(stats: &HarStats, ...) -> Vec<String>` — CI/CD validation
- `generate_report(har: &HarFile, stats: &HarStats, format: &str) -> String` — Render reports
- `suggest_filename(path: &str, suffix: &str) -> String` — Generate output filenames

## License

MIT