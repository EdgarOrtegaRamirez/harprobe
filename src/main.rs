use clap::{Parser, Subcommand};
use harprobe::*;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "harprobe", version, about = "HTTP Archive (HAR) file analysis CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze a HAR file and print statistics
    Stats {
        /// Path to the HAR file
        file: PathBuf,
        /// Output format (text, json, markdown) [default: text]
        #[arg(long, default_value = "text", value_parser = ["text", "json", "markdown"])]
        format: String,
        /// Save output to file instead of stdout
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// List the slowest requests in a HAR file
    Slowest {
        /// Path to the HAR file
        file: PathBuf,
        /// Number of slowest entries to show [default: 10]
        #[arg(long, default_value = "10")]
        count: usize,
    },
    /// Filter entries in a HAR file
    Filter {
        /// Path to the HAR file
        file: PathBuf,
        /// Minimum status code
        #[arg(long)]
        status_min: Option<i64>,
        /// Maximum status code
        #[arg(long)]
        status_max: Option<i64>,
        /// URL pattern to match (substring)
        #[arg(long)]
        url: Option<String>,
        /// MIME type to match (substring)
        #[arg(long)]
        mime: Option<String>,
        /// Minimum response time in ms
        #[arg(long)]
        min_time: Option<f64>,
        /// Maximum response time in ms
        #[arg(long)]
        max_time: Option<f64>,
    },
    /// Diff two HAR files
    Diff {
        /// First HAR file (before)
        before: PathBuf,
        /// Second HAR file (after)
        after: PathBuf,
        /// Output format (text, json) [default: text]
        #[arg(long, default_value = "text", value_parser = ["text", "json"])]
        format: String,
    },
    /// CI/CD mode — exit with non-zero if thresholds are exceeded
    Ci {
        /// Path to the HAR file
        file: PathBuf,
        /// Maximum total time in ms
        #[arg(long)]
        max_total_time: Option<f64>,
        /// Maximum number of requests
        #[arg(long)]
        max_requests: Option<usize>,
        /// Maximum number of error responses (4xx/5xx)
        #[arg(long)]
        max_errors: Option<usize>,
        /// Maximum total transfer size in bytes
        #[arg(long)]
        max_total_size: Option<i64>,
    },
    /// Show HAR file metadata
    Info {
        /// Path to the HAR file
        file: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Stats { file, format, output } => {
            let content = match fs::read_to_string(&file) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error reading file '{}': {}", file.display(), e);
                    std::process::exit(1);
                }
            };
            let har = match parse_har(&content) {
                Ok(h) => h,
                Err(e) => {
                    eprintln!("Error parsing HAR: {}", e);
                    std::process::exit(1);
                }
            };
            let stats = analyze(&har);

            let report = match format.as_str() {
                "json" => format_json_report(&stats, None),
                "markdown" => format_markdown_report(&stats, &har),
                _ => format_text_report(&stats, &har),
            };

            match output {
                Some(path) => {
                    if let Err(e) = fs::write(&path, &report) {
                        eprintln!("Error writing to '{}': {}", path.display(), e);
                        std::process::exit(1);
                    }
                    println!("Report written to {}", path.display());
                }
                None => print!("{}", report),
            }
        }

        Commands::Slowest { file, count } => {
            let content = match fs::read_to_string(&file) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error reading file '{}': {}", file.display(), e);
                    std::process::exit(1);
                }
            };
            let har = match parse_har(&content) {
                Ok(h) => h,
                Err(e) => {
                    eprintln!("Error parsing HAR: {}", e);
                    std::process::exit(1);
                }
            };
            let stats = analyze(&har);

            println!("🐌 {} Slowest Requests\n", stats.slowest_entries.len().min(count));
            for (i, entry) in stats.slowest_entries.iter().take(count).enumerate() {
                let status_color = if entry.status >= 400 { "✗" } else if entry.status >= 300 { "→" } else { "✓" };
                println!("{:>2}. {} {:>3} {:>8.0}ms {:>8} {} {}",
                    i + 1, status_color, entry.status, entry.total_time_ms,
                    format_size(entry.size), entry.method, entry.url);
            }
        }

        Commands::Filter { file, status_min, status_max, url, mime, min_time, max_time } => {
            let content = match fs::read_to_string(&file) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error reading file '{}': {}", file.display(), e);
                    std::process::exit(1);
                }
            };
            let har = match parse_har(&content) {
                Ok(h) => h,
                Err(e) => {
                    eprintln!("Error parsing HAR: {}", e);
                    std::process::exit(1);
                }
            };

            let filtered = filter_entries(&har.log.entries, status_min, status_max,
                url.as_deref(), mime.as_deref(), min_time, max_time);

            println!("Filtered {} of {} entries\n", filtered.len(), har.log.entries.len());
            for entry in filtered {
                let entry_time = entry.timings.wait + entry.timings.receive;
                println!("{:>6} {:>3} {:>6.0}ms {}",
                    entry.request.method, entry.response.status, entry_time, entry.request.url);
            }
        }

        Commands::Diff { before, after, format } => {
            let before_content = match fs::read_to_string(&before) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error reading '{}': {}", before.display(), e);
                    std::process::exit(1);
                }
            };
            let after_content = match fs::read_to_string(&after) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error reading '{}': {}", after.display(), e);
                    std::process::exit(1);
                }
            };

            let before_har = match parse_har(&before_content) {
                Ok(h) => h,
                Err(e) => { eprintln!("Error parsing '{}': {}", before.display(), e); std::process::exit(1); }
            };
            let after_har = match parse_har(&after_content) {
                Ok(h) => h,
                Err(e) => { eprintln!("Error parsing '{}': {}", after.display(), e); std::process::exit(1); }
            };

            let diff = diff_har(&before_har, &after_har);

            match format.as_str() {
                "json" => {
                    let report = format_json_report(&analyze(&after_har), Some(&diff));
                    println!("{}", report);
                }
                _ => {
                    println!("📊 HAR Diff Report\n");
                    println!("  Before: {} entries", before_har.log.entries.len());
                    println!("  After:  {} entries", after_har.log.entries.len());
                    println!("  Change: {:+} entries\n", diff.request_count_change);

                    println!("  Added:     {}", diff.added_entries.len());
                    println!("  Removed:   {}", diff.removed_entries.len());
                    println!("  Unchanged: {}\n", diff.same_entries);

                    let direction = if diff.total_time_change_ms >= 0.0 { "+" } else { "" };
                    println!("  Total time change: {}{:.0}ms", direction, diff.total_time_change_ms);
                    let size_direction = if diff.total_size_change >= 0 { "+" } else { "" };
                    println!("  Total size change: {}{}", size_direction, format_size(diff.total_size_change));

                    if !diff.added_entries.is_empty() {
                        println!("\n  ✚ Added Entries:");
                        for entry in &diff.added_entries {
                            println!("    {:>6} {:>3} {:>8.0}ms {}",
                                entry.method, entry.status, entry.total_time_ms, entry.url);
                        }
                    }
                    if !diff.removed_entries.is_empty() {
                        println!("\n  ✖ Removed Entries:");
                        for entry in &diff.removed_entries {
                            println!("    {:>6} {:>3} {:>8.0}ms {}",
                                entry.method, entry.status, entry.total_time_ms, entry.url);
                        }
                    }
                }
            }
        }

        Commands::Ci { file, max_total_time, max_requests, max_errors, max_total_size } => {
            let content = match fs::read_to_string(&file) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error reading file '{}': {}", file.display(), e);
                    std::process::exit(1);
                }
            };
            let har = match parse_har(&content) {
                Ok(h) => h,
                Err(e) => {
                    eprintln!("Error parsing HAR: {}", e);
                    std::process::exit(1);
                }
            };
            let stats = analyze(&har);
            let violations = check_thresholds(&stats, max_total_time, max_requests, max_errors, max_total_size);

            if violations.is_empty() {
                println!("✅ All thresholds passed");
                std::process::exit(0);
            } else {
                for v in &violations {
                    eprintln!("❌ {}", v);
                }
                std::process::exit(1);
            }
        }

        Commands::Info { file } => {
            let content = match fs::read_to_string(&file) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error reading file '{}': {}", file.display(), e);
                    std::process::exit(1);
                }
            };
            let har = match parse_har(&content) {
                Ok(h) => h,
                Err(e) => {
                    eprintln!("Error parsing HAR: {}", e);
                    std::process::exit(1);
                }
            };

            println!("📄 HAR File Information\n");
            println!("  Creator:     {} v{}", har.log.creator.name, har.log.creator.version);
            if let Some(ref browser) = har.log.browser {
                println!("  Browser:     {} v{}", browser.name, browser.version);
            }
            println!("  HAR Version: {}", har.log.version);
            println!("  Pages:       {}", har.log.pages.len());
            println!("  Entries:     {}", har.log.entries.len());

            if !har.log.pages.is_empty() {
                println!("\n  Pages:");
                for page in &har.log.pages {
                    let on_load = page.page_timings.as_ref()
                        .and_then(|t| t.on_load)
                        .map(|t| format!("{:.0}ms", t))
                        .unwrap_or_else(|| "N/A".to_string());
                    println!("    [{}] {} (onLoad: {})", page.id, page.title, on_load);
                }
            }
        }
    }
}