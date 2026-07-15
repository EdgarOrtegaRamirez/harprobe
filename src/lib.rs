#![allow(
    clippy::useless_format,
    clippy::single_char_add_str,
    clippy::unnecessary_sort_by
)]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Top-level HAR file structure (HAR 1.2 spec)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HarFile {
    pub log: Log,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Log {
    pub version: String,
    pub creator: Creator,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser: Option<Browser>,
    pub pages: Vec<Page>,
    pub entries: Vec<Entry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Creator {
    pub name: String,
    pub version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Browser {
    pub name: String,
    pub version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Page {
    #[serde(default)]
    pub started_date_time: String,
    pub id: String,
    pub title: String,
    #[serde(
        rename = "pageTimings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub page_timings: Option<PageTimings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageTimings {
    #[serde(
        rename = "onContentLoad",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub on_content_load: Option<f64>,
    #[serde(rename = "onLoad", default, skip_serializing_if = "Option::is_none")]
    pub on_load: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entry {
    #[serde(default)]
    pub pageref: Option<String>,
    #[serde(default)]
    pub started_date_time: String,
    pub request: Request,
    pub response: Response,
    pub cache: Cache,
    pub timings: Timings,
    #[serde(default)]
    pub server_ip_address: Option<String>,
    #[serde(default)]
    pub connection: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(default)]
    pub _time: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Request {
    pub method: String,
    pub url: String,
    #[serde(default)]
    pub http_version: String,
    pub headers: Vec<Header>,
    #[serde(default)]
    pub query_string: Vec<QueryString>,
    #[serde(default)]
    pub cookies: Vec<Cookie>,
    #[serde(default)]
    pub headers_size: i64,
    #[serde(default)]
    pub body_size: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_data: Option<PostData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response {
    pub status: i64,
    #[serde(default)]
    pub status_text: String,
    #[serde(default)]
    pub http_version: String,
    pub headers: Vec<Header>,
    pub cookies: Vec<Cookie>,
    #[serde(default)]
    pub content: Content,
    #[serde(rename = "redirectURL", default)]
    pub redirect_url: String,
    #[serde(rename = "headersSize", default)]
    pub headers_size: i64,
    #[serde(rename = "bodySize", default)]
    pub body_size: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Header {
    pub name: String,
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryString {
    pub name: String,
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub domain: Option<String>,
    #[serde(default)]
    pub expires: Option<String>,
    #[serde(default)]
    pub http_only: Option<bool>,
    #[serde(default)]
    pub secure: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Content {
    #[serde(default)]
    pub size: i64,
    #[serde(rename = "mimeType", default)]
    pub mime_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compression: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostData {
    pub mime_type: String,
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<Param>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Param {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cache {
    #[serde(
        rename = "beforeRequest",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub before_request: Option<CacheEntry>,
    #[serde(
        rename = "afterRequest",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub after_request: Option<CacheEntry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CacheEntry {
    #[serde(default)]
    pub expires: Option<String>,
    #[serde(default)]
    pub last_access: String,
    #[serde(default)]
    pub e_tag: String,
    #[serde(default)]
    pub hit_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Timings {
    #[serde(default)]
    pub blocked: Option<f64>,
    #[serde(default)]
    pub dns: Option<f64>,
    #[serde(default)]
    pub connect: Option<f64>,
    #[serde(default)]
    pub send: f64,
    #[serde(default)]
    pub wait: f64,
    #[serde(default)]
    pub receive: f64,
    #[serde(default)]
    pub ssl: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

// ============================================================
// Analysis types
// ============================================================

#[derive(Debug, Clone, Serialize)]
pub struct HarStats {
    pub total_entries: usize,
    pub total_pages: usize,
    pub total_transfer_size: i64,
    pub total_body_size: i64,
    pub total_headers_size: i64,
    pub total_time_ms: f64,
    pub status_code_distribution: HashMap<i64, usize>,
    pub content_type_distribution: HashMap<String, usize>,
    pub method_distribution: HashMap<String, usize>,
    pub cache_hits: usize,
    pub cache_misses: usize,
    pub slowest_entries: Vec<EntrySummary>,
    pub page_load_times: Vec<PageLoadSummary>,
    pub per_domain_stats: HashMap<String, DomainStats>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EntrySummary {
    pub url: String,
    pub method: String,
    pub status: i64,
    pub total_time_ms: f64,
    pub size: i64,
    pub mime_type: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PageLoadSummary {
    pub page_id: String,
    pub title: String,
    pub on_content_load: Option<f64>,
    pub on_load: Option<f64>,
    pub entries: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct DomainStats {
    pub requests: usize,
    pub total_time_ms: f64,
    pub total_size: i64,
    pub errors: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct HarDiff {
    pub added_entries: Vec<EntrySummary>,
    pub removed_entries: Vec<EntrySummary>,
    pub same_entries: usize,
    pub total_time_change_ms: f64,
    pub total_size_change: i64,
    pub request_count_change: i64,
    pub status_code_changes: HashMap<i64, (usize, usize)>, // (before, after)
}

// ============================================================
// Analysis functions
// ============================================================

/// Parse a HAR file from a JSON string
pub fn parse_har(content: &str) -> Result<HarFile, HarProbeError> {
    serde_json::from_str::<HarFile>(content).map_err(|e| HarProbeError::ParseError(e.to_string()))
}

/// Compute comprehensive statistics for a HAR file
pub fn analyze(har: &HarFile) -> HarStats {
    let mut status_codes: HashMap<i64, usize> = HashMap::new();
    let mut content_types: HashMap<String, usize> = HashMap::new();
    let mut methods: HashMap<String, usize> = HashMap::new();
    let mut cache_hits = 0usize;
    let mut cache_misses = 0usize;
    let mut total_time = 0.0f64;
    let mut total_transfer = 0i64;
    let mut total_body = 0i64;
    let mut total_headers = 0i64;
    let mut domain_map: HashMap<String, DomainStats> = HashMap::new();
    let mut slowest: Vec<EntrySummary> = Vec::new();

    for entry in &har.log.entries {
        // Status codes
        *status_codes.entry(entry.response.status).or_insert(0) += 1;

        // Content types
        let mime = entry
            .response
            .content
            .mime_type
            .split(';')
            .next()
            .unwrap_or("unknown")
            .to_string();
        *content_types.entry(mime.clone()).or_insert(0) += 1;

        // Methods
        *methods.entry(entry.request.method.clone()).or_insert(0) += 1;

        // Cache
        if entry.cache.after_request.is_some() {
            cache_hits += 1;
        } else {
            cache_misses += 1;
        }

        // Timing
        let entry_time = entry.timings.wait
            + entry.timings.receive
            + entry.timings.send
            + entry.timings.dns.unwrap_or(0.0)
            + entry.timings.connect.unwrap_or(0.0)
            + entry.timings.blocked.unwrap_or(0.0)
            + entry.timings.ssl.unwrap_or(0.0);
        total_time += entry_time;

        // Sizes
        let header_size = entry.response.headers_size.max(0);
        let body_size = entry.response.body_size.max(0);
        total_headers += header_size;
        total_body += body_size;
        total_transfer += header_size + body_size;

        // Domain stats
        if let Some(domain) = extract_domain(&entry.request.url) {
            let ds = domain_map.entry(domain).or_insert(DomainStats {
                requests: 0,
                total_time_ms: 0.0,
                total_size: 0,
                errors: 0,
            });
            ds.requests += 1;
            ds.total_time_ms += entry_time;
            ds.total_size += header_size + body_size;
            if entry.response.status >= 400 {
                ds.errors += 1;
            }
        }

        // Track slowest
        slowest.push(EntrySummary {
            url: entry.request.url.clone(),
            method: entry.request.method.clone(),
            status: entry.response.status,
            total_time_ms: entry_time,
            size: header_size + body_size,
            mime_type: mime,
        });
    }

    // Sort slowest
    slowest.sort_by(|a, b| {
        b.total_time_ms
            .partial_cmp(&a.total_time_ms)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // Page load times
    let page_loads: Vec<PageLoadSummary> = har
        .log
        .pages
        .iter()
        .map(|page| {
            let entry_count = har
                .log
                .entries
                .iter()
                .filter(|e| e.pageref.as_deref() == Some(&page.id))
                .count();
            PageLoadSummary {
                page_id: page.id.clone(),
                title: page.title.clone(),
                on_content_load: page.page_timings.as_ref().and_then(|t| t.on_content_load),
                on_load: page.page_timings.as_ref().and_then(|t| t.on_load),
                entries: entry_count,
            }
        })
        .collect();

    HarStats {
        total_entries: har.log.entries.len(),
        total_pages: har.log.pages.len(),
        total_transfer_size: total_transfer,
        total_body_size: total_body,
        total_headers_size: total_headers,
        total_time_ms: total_time,
        status_code_distribution: status_codes,
        content_type_distribution: content_types,
        method_distribution: methods,
        cache_hits,
        cache_misses,
        slowest_entries: slowest,
        page_load_times: page_loads,
        per_domain_stats: domain_map,
    }
}

/// Diff two HAR files
pub fn diff_har(before: &HarFile, after: &HarFile) -> HarDiff {
    // Build a key-based lookup for each entry (URL + method)
    let before_map: HashMap<(&str, &str), &Entry> = before
        .log
        .entries
        .iter()
        .map(|e| ((e.request.url.as_str(), e.request.method.as_str()), e))
        .collect();
    let after_map: HashMap<(&str, &str), &Entry> = after
        .log
        .entries
        .iter()
        .map(|e| ((e.request.url.as_str(), e.request.method.as_str()), e))
        .collect();

    let mut added = Vec::new();
    let mut removed = Vec::new();
    let mut same = 0usize;
    let mut total_time_change = 0.0f64;
    let mut total_size_change = 0i64;

    // Find added and same entries
    for entry in &after.log.entries {
        let key = (entry.request.url.as_str(), entry.request.method.as_str());
        match before_map.get(&key) {
            Some(before_entry) => {
                same += 1;
                let before_time = before_entry.timings.wait + before_entry.timings.receive;
                let after_time = entry.timings.wait + entry.timings.receive;
                total_time_change += after_time - before_time;
                let before_size = before_entry.response.body_size.max(0)
                    + before_entry.response.headers_size.max(0);
                let after_size =
                    entry.response.body_size.max(0) + entry.response.headers_size.max(0);
                total_size_change += after_size - before_size;
            }
            None => {
                let entry_time = entry.timings.wait
                    + entry.timings.receive
                    + entry.timings.send
                    + entry.timings.dns.unwrap_or(0.0)
                    + entry.timings.connect.unwrap_or(0.0);
                added.push(EntrySummary {
                    url: entry.request.url.clone(),
                    method: entry.request.method.clone(),
                    status: entry.response.status,
                    total_time_ms: entry_time,
                    size: entry.response.body_size.max(0) + entry.response.headers_size.max(0),
                    mime_type: entry
                        .response
                        .content
                        .mime_type
                        .split(';')
                        .next()
                        .unwrap_or("unknown")
                        .to_string(),
                });
            }
        }
    }

    // Find removed entries
    for entry in &before.log.entries {
        let key = (entry.request.url.as_str(), entry.request.method.as_str());
        if !after_map.contains_key(&key) {
            let entry_time = entry.timings.wait
                + entry.timings.receive
                + entry.timings.send
                + entry.timings.dns.unwrap_or(0.0)
                + entry.timings.connect.unwrap_or(0.0);
            removed.push(EntrySummary {
                url: entry.request.url.clone(),
                method: entry.request.method.clone(),
                status: entry.response.status,
                total_time_ms: entry_time,
                size: entry.response.body_size.max(0) + entry.response.headers_size.max(0),
                mime_type: entry
                    .response
                    .content
                    .mime_type
                    .split(';')
                    .next()
                    .unwrap_or("unknown")
                    .to_string(),
            });
        }
    }

    // Status code changes
    let mut sc_changes: HashMap<i64, (usize, usize)> = HashMap::new();
    let before_sc: HashMap<i64, usize> =
        before
            .log
            .entries
            .iter()
            .fold(HashMap::new(), |mut acc, e| {
                *acc.entry(e.response.status).or_insert(0) += 1;
                acc
            });
    let after_sc: HashMap<i64, usize> =
        after.log.entries.iter().fold(HashMap::new(), |mut acc, e| {
            *acc.entry(e.response.status).or_insert(0) += 1;
            acc
        });
    let all_codes: Vec<i64> = before_sc
        .keys()
        .chain(after_sc.keys())
        .copied()
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    for code in all_codes {
        let b = before_sc.get(&code).copied().unwrap_or(0);
        let a = after_sc.get(&code).copied().unwrap_or(0);
        if b != a {
            sc_changes.insert(code, (b, a));
        }
    }

    HarDiff {
        added_entries: added,
        removed_entries: removed,
        same_entries: same,
        total_time_change_ms: total_time_change,
        total_size_change,
        request_count_change: after.log.entries.len() as i64 - before.log.entries.len() as i64,
        status_code_changes: sc_changes,
    }
}

/// Filter entries by various criteria
pub fn filter_entries<'a>(
    entries: &'a [Entry],
    status_min: Option<i64>,
    status_max: Option<i64>,
    url_pattern: Option<&str>,
    mime_filter: Option<&str>,
    min_time: Option<f64>,
    max_time: Option<f64>,
) -> Vec<&'a Entry> {
    entries
        .iter()
        .filter(|e| {
            let mut pass = true;
            if let Some(smin) = status_min {
                if e.response.status < smin {
                    pass = false;
                }
            }
            if let Some(smax) = status_max {
                if e.response.status > smax {
                    pass = false;
                }
            }
            if let Some(pattern) = url_pattern {
                if !e.request.url.contains(pattern) {
                    pass = false;
                }
            }
            if let Some(mime) = mime_filter {
                if !e.response.content.mime_type.contains(mime) {
                    pass = false;
                }
            }
            if let Some(tmin) = min_time {
                let entry_time = e.timings.wait + e.timings.receive;
                if entry_time < tmin {
                    pass = false;
                }
            }
            if let Some(tmax) = max_time {
                let entry_time = e.timings.wait + e.timings.receive;
                if entry_time > tmax {
                    pass = false;
                }
            }
            pass
        })
        .collect()
}

/// Check CI/CD thresholds
pub fn check_thresholds(
    stats: &HarStats,
    max_total_time: Option<f64>,
    max_requests: Option<usize>,
    max_errors: Option<usize>,
    max_total_size: Option<i64>,
) -> Vec<String> {
    let mut violations = Vec::new();

    if let Some(mtt) = max_total_time {
        let total_ms = stats.total_time_ms;
        if total_ms > mtt {
            violations.push(format!(
                "Total time {:.0}ms exceeds threshold {:.0}ms",
                total_ms, mtt
            ));
        }
    }
    if let Some(mr) = max_requests {
        if stats.total_entries > mr {
            violations.push(format!(
                "Request count {} exceeds threshold {}",
                stats.total_entries, mr
            ));
        }
    }
    if let Some(me) = max_errors {
        let errors: usize = stats
            .status_code_distribution
            .iter()
            .filter(|(k, _)| **k >= 400)
            .map(|(_, v)| v)
            .sum();
        if errors > me {
            violations.push(format!("Error count {} exceeds threshold {}", errors, me));
        }
    }
    if let Some(mts) = max_total_size {
        if stats.total_transfer_size > mts {
            violations.push(format!(
                "Transfer size {} bytes exceeds threshold {} bytes",
                stats.total_transfer_size, mts
            ));
        }
    }

    violations
}

/// Generate a text report
pub fn format_text_report(stats: &HarStats, har: &HarFile) -> String {
    let mut out = String::new();

    out.push_str(&format!("╔══════════════════════════════════════╗\n"));
    out.push_str(&format!("║         HAR Analysis Report          ║\n"));
    out.push_str(&format!("╚══════════════════════════════════════╝\n\n"));

    out.push_str(&format!(
        "Creator: {} v{}\n",
        har.log.creator.name, har.log.creator.version
    ));
    if let Some(ref browser) = har.log.browser {
        out.push_str(&format!("Browser: {} v{}\n", browser.name, browser.version));
    }
    out.push_str(&format!("HAR Version: {}\n\n", har.log.version));

    out.push_str(&format!("📊 OVERVIEW\n"));
    out.push_str(&format!("───────────\n"));
    out.push_str(&format!("  Total requests:     {}\n", stats.total_entries));
    out.push_str(&format!("  Total pages:        {}\n", stats.total_pages));
    out.push_str(&format!(
        "  Total transfer:     {} bytes ({} KB)\n",
        stats.total_transfer_size,
        stats.total_transfer_size / 1024
    ));
    out.push_str(&format!(
        "  Total body size:    {} bytes ({} KB)\n",
        stats.total_body_size,
        stats.total_body_size / 1024
    ));
    out.push_str(&format!(
        "  Total headers size: {} bytes ({} KB)\n",
        stats.total_headers_size,
        stats.total_headers_size / 1024
    ));
    out.push_str(&format!(
        "  Total time:         {:.0} ms\n\n",
        stats.total_time_ms
    ));

    out.push_str(&format!("📊 STATUS CODE DISTRIBUTION\n"));
    out.push_str(&format!("────────────────────────────\n"));
    let mut codes: Vec<_> = stats.status_code_distribution.iter().collect();
    codes.sort_by_key(|(k, _)| **k);
    for (code, count) in &codes {
        let pct = (**count as f64 / stats.total_entries as f64) * 100.0;
        let bar = "█".repeat((pct / 2.0) as usize);
        let _color = if **code >= 400 {
            "✗"
        } else if **code >= 300 {
            "→"
        } else {
            "✓"
        };
        out.push_str(&format!(
            "  {:>3} {:>4} ({:>5.1}%) {}\n",
            code, count, bar, pct
        ));
    }
    out.push_str("\n");

    out.push_str(&format!("📊 CONTENT TYPE DISTRIBUTION\n"));
    out.push_str(&format!("──────────────────────────────\n"));
    let mut types: Vec<_> = stats.content_type_distribution.iter().collect();
    types.sort_by(|a, b| b.1.cmp(a.1));
    for (mime, count) in &types {
        let pct = (**count as f64 / stats.total_entries as f64) * 100.0;
        out.push_str(&format!("  {:>4} ({:>5.1}%) {}\n", count, pct, mime));
    }
    out.push_str("\n");

    out.push_str(&format!("📊 METHOD DISTRIBUTION\n"));
    out.push_str(&format!("───────────────────────\n"));
    let mut methods: Vec<_> = stats.method_distribution.iter().collect();
    methods.sort_by(|a, b| b.1.cmp(a.1));
    for (method, count) in &methods {
        let pct = (**count as f64 / stats.total_entries as f64) * 100.0;
        out.push_str(&format!("  {:>6} {:>4} ({:>5.1}%)\n", method, count, pct));
    }
    out.push_str("\n");

    out.push_str(&format!("📊 CACHE PERFORMANCE\n"));
    out.push_str(&format!("─────────────────────\n"));
    out.push_str(&format!("  Cache hits:  {}\n", stats.cache_hits));
    out.push_str(&format!("  Cache misses: {}\n", stats.cache_misses));
    if stats.cache_hits + stats.cache_misses > 0 {
        let hit_pct =
            (stats.cache_hits as f64 / (stats.cache_hits + stats.cache_misses) as f64) * 100.0;
        out.push_str(&format!("  Hit rate:    {:.1}%\n", hit_pct));
    }
    out.push_str("\n");

    if !stats.slowest_entries.is_empty() {
        out.push_str(&format!("🐌 SLOWEST REQUESTS (top 10)\n"));
        out.push_str(&format!("────────────────────────────\n"));
        for (i, entry) in stats.slowest_entries.iter().take(10).enumerate() {
            let status_color = if entry.status >= 400 {
                "✗"
            } else if entry.status >= 300 {
                "→"
            } else {
                "✓"
            };
            let url_short = if entry.url.len() > 80 {
                format!(
                    "{}...{}",
                    &entry.url[..60],
                    &entry.url[entry.url.len() - 20..]
                )
            } else {
                entry.url.clone()
            };
            out.push_str(&format!(
                "  {:>2}. {} {:>3} {:>8.0}ms {:>8} {} {}\n",
                i + 1,
                status_color,
                entry.status,
                entry.total_time_ms,
                format_size(entry.size),
                entry.method,
                url_short
            ));
        }
        out.push_str("\n");
    }

    if !stats.per_domain_stats.is_empty() {
        out.push_str(&format!("🌐 PER-DOMAIN STATS\n"));
        out.push_str(&format!("───────────────────\n"));
        let mut domains: Vec<_> = stats.per_domain_stats.iter().collect();
        domains.sort_by(|a, b| b.1.requests.cmp(&a.1.requests));
        for (domain, ds) in &domains {
            let avg_time = if ds.requests > 0 {
                ds.total_time_ms / ds.requests as f64
            } else {
                0.0
            };
            out.push_str(&format!(
                "  {:>4} reqs {:>10.0}ms avg {:>10} total {:>4} errs {}\n",
                ds.requests,
                avg_time,
                format_size(ds.total_size),
                ds.errors,
                domain
            ));
        }
        out.push_str("\n");
    }

    if !stats.page_load_times.is_empty() {
        out.push_str(&format!("📄 PAGE LOAD TIMES\n"));
        out.push_str(&format!("───────────────────\n"));
        for pl in &stats.page_load_times {
            out.push_str(&format!("  [{}] {}\n", pl.page_id, pl.title));
            out.push_str(&format!(
                "       Entries: {}, DOM Content: {:.0}ms, On Load: {:.0}ms\n",
                pl.entries,
                pl.on_content_load.unwrap_or(0.0),
                pl.on_load.unwrap_or(0.0)
            ));
        }
        out.push_str("\n");
    }

    out
}

/// Generate a JSON report
pub fn format_json_report(stats: &HarStats, diff: Option<&HarDiff>) -> String {
    let report = serde_json::json!({
        "stats": {
            "total_entries": stats.total_entries,
            "total_pages": stats.total_pages,
            "total_transfer_size_bytes": stats.total_transfer_size,
            "total_body_size_bytes": stats.total_body_size,
            "total_headers_size_bytes": stats.total_headers_size,
            "total_time_ms": stats.total_time_ms,
            "status_codes": stats.status_code_distribution,
            "content_types": stats.content_type_distribution,
            "methods": stats.method_distribution,
            "cache_hits": stats.cache_hits,
            "cache_misses": stats.cache_misses,
            "slowest_entries": &stats.slowest_entries.iter().take(10).collect::<Vec<_>>(),
            "domains": stats.per_domain_stats,
            "pages": stats.page_load_times,
        },
        "diff": diff.map(|d| serde_json::json!({
            "added": d.added_entries.len(),
            "removed": d.removed_entries.len(),
            "same": d.same_entries,
            "total_time_change_ms": d.total_time_change_ms,
            "total_size_change_bytes": d.total_size_change,
            "request_count_change": d.request_count_change,
            "status_code_changes": d.status_code_changes,
        })),
    });
    serde_json::to_string_pretty(&report).unwrap_or_default()
}

/// Generate a markdown report
pub fn format_markdown_report(stats: &HarStats, har: &HarFile) -> String {
    let mut out = String::new();

    out.push_str(&format!("# HAR Analysis Report\n\n"));
    out.push_str(&format!(
        "- **Creator**: {} v{}\n",
        har.log.creator.name, har.log.creator.version
    ));
    if let Some(ref browser) = har.log.browser {
        out.push_str(&format!(
            "- **Browser**: {} v{}\n",
            browser.name, browser.version
        ));
    }
    out.push_str(&format!("- **HAR Version**: {}\n", har.log.version));
    out.push_str("\n");

    out.push_str("## Overview\n\n");
    out.push_str(&format!("| Metric | Value |\n"));
    out.push_str(&format!("|--------|-------|\n"));
    out.push_str(&format!("| Total Requests | {} |\n", stats.total_entries));
    out.push_str(&format!("| Total Pages | {} |\n", stats.total_pages));
    out.push_str(&format!(
        "| Total Transfer | {} |\n",
        format_size(stats.total_transfer_size)
    ));
    out.push_str(&format!("| Total Time | {:.0} ms |\n", stats.total_time_ms));
    out.push_str("\n");

    out.push_str("## Status Codes\n\n");
    out.push_str("| Code | Count | Percentage |\n");
    out.push_str("|------|-------|------------|\n");
    let mut codes: Vec<_> = stats.status_code_distribution.iter().collect();
    codes.sort_by_key(|(k, _)| **k);
    for (code, count) in &codes {
        let pct = (**count as f64 / stats.total_entries as f64) * 100.0;
        out.push_str(&format!("| {} | {} | {:.1}% |\n", code, count, pct));
    }
    out.push_str("\n");

    if !stats.slowest_entries.is_empty() {
        out.push_str("## Slowest Requests (Top 10)\n\n");
        out.push_str("| # | Status | Time | Size | Method | URL |\n");
        out.push_str("|---|--------|------|------|--------|-----|\n");
        for (i, entry) in stats.slowest_entries.iter().take(10).enumerate() {
            out.push_str(&format!(
                "| {} | {} | {:.0}ms | {} | {} | {} |\n",
                i + 1,
                entry.status,
                entry.total_time_ms,
                format_size(entry.size),
                entry.method,
                entry.url
            ));
        }
        out.push_str("\n");
    }

    out
}

/// Format a filename from a HAR file path
pub fn suggest_filename(path: &str, suffix: &str) -> String {
    let stem = std::path::Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("har");
    format!(
        "{}-{}.{}",
        stem,
        suffix,
        if suffix == "json" { "json" } else { "md" }
    )
}

// ============================================================
// Helpers
// ============================================================

fn extract_domain(url: &str) -> Option<String> {
    url.split('/').nth(2).map(|s| s.to_lowercase())
}

pub fn format_size(size: i64) -> String {
    if size < 1024 {
        format!("{}B", size)
    } else if size < 1024 * 1024 {
        format!("{:.1}KB", size as f64 / 1024.0)
    } else {
        format!("{:.1}MB", size as f64 / (1024.0 * 1024.0))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum HarProbeError {
    #[error("Failed to parse HAR file: {0}")]
    ParseError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_har() -> HarFile {
        serde_json::from_str(
            r#"{
            "log": {
                "version": "1.2",
                "creator": { "name": "test", "version": "1.0" },
                "pages": [
                    {
                        "startedDateTime": "2024-01-01T00:00:00.000Z",
                        "id": "page_1",
                        "title": "Test Page",
                        "pageTimings": { "onContentLoad": 100, "onLoad": 300 }
                    }
                ],
                "entries": [
                    {
                        "pageref": "page_1",
                        "startedDateTime": "2024-01-01T00:00:00.000Z",
                        "request": {
                            "method": "GET",
                            "url": "https://example.com/api/data",
                            "httpVersion": "HTTP/1.1",
                            "headers": [{"name": "Accept", "value": "application/json"}],
                            "queryString": [],
                            "cookies": [],
                            "headersSize": 100,
                            "bodySize": 0
                        },
                        "response": {
                            "status": 200,
                            "statusText": "OK",
                            "httpVersion": "HTTP/1.1",
                            "headers": [{"name": "Content-Type", "value": "application/json"}],
                            "cookies": [],
                            "content": {"size": 500, "mimeType": "application/json"},
                            "redirectURL": "",
                            "headersSize": 200,
                            "bodySize": 500
                        },
                        "cache": {},
                        "timings": {
                            "blocked": 5.0, "dns": 10.0, "connect": 20.0,
                            "send": 2.0, "wait": 150.0, "receive": 30.0, "ssl": 15.0
                        },
                        "_time": 232.0
                    },
                    {
                        "pageref": "page_1",
                        "startedDateTime": "2024-01-01T00:00:01.000Z",
                        "request": {
                            "method": "POST",
                            "url": "https://example.com/api/submit",
                            "httpVersion": "HTTP/1.1",
                            "headers": [{"name": "Accept", "value": "application/json"}],
                            "queryString": [],
                            "cookies": [],
                            "headersSize": 120,
                            "bodySize": 200
                        },
                        "response": {
                            "status": 201,
                            "statusText": "Created",
                            "httpVersion": "HTTP/1.1",
                            "headers": [{"name": "Content-Type", "value": "application/json"}],
                            "cookies": [],
                            "content": {"size": 300, "mimeType": "application/json"},
                            "redirectURL": "",
                            "headersSize": 180,
                            "bodySize": 300
                        },
                        "cache": {},
                        "timings": {
                            "blocked": null, "dns": null, "connect": null,
                            "send": 1.0, "wait": 80.0, "receive": 20.0, "ssl": null
                        },
                        "_time": 101.0
                    },
                    {
                        "pageref": "page_1",
                        "startedDateTime": "2024-01-01T00:00:02.000Z",
                        "request": {
                            "method": "GET",
                            "url": "https://cdn.example.com/style.css",
                            "httpVersion": "HTTP/1.1",
                            "headers": [{"name": "Accept", "value": "text/css"}],
                            "queryString": [],
                            "cookies": [],
                            "headersSize": 90,
                            "bodySize": 0
                        },
                        "response": {
                            "status": 200,
                            "statusText": "OK",
                            "httpVersion": "HTTP/1.1",
                            "headers": [{"name": "Content-Type", "value": "text/css"}],
                            "cookies": [],
                            "content": {"size": 10000, "mimeType": "text/css"},
                            "redirectURL": "",
                            "headersSize": 150,
                            "bodySize": 10000
                        },
                        "cache": {
                            "afterRequest": {
                                "expires": "2024-02-01T00:00:00.000Z",
                                "lastAccess": "2024-01-01T00:00:00.000Z",
                                "eTag": "\"abc123\"",
                                "hitCount": 1
                            }
                        },
                        "timings": {
                            "blocked": 2.0, "dns": 5.0, "connect": 10.0,
                            "send": 1.0, "wait": 40.0, "receive": 15.0, "ssl": 8.0
                        },
                        "_time": 81.0
                    },
                    {
                        "pageref": "page_1",
                        "startedDateTime": "2024-01-01T00:00:03.000Z",
                        "request": {
                            "method": "GET",
                            "url": "https://api.example.com/error",
                            "httpVersion": "HTTP/1.1",
                            "headers": [{"name": "Accept", "value": "application/json"}],
                            "queryString": [],
                            "cookies": [],
                            "headersSize": 100,
                            "bodySize": 0
                        },
                        "response": {
                            "status": 500,
                            "statusText": "Server Error",
                            "httpVersion": "HTTP/1.1",
                            "headers": [{"name": "Content-Type", "value": "application/json"}],
                            "cookies": [],
                            "content": {"size": 100, "mimeType": "application/json"},
                            "redirectURL": "",
                            "headersSize": 150,
                            "bodySize": 100
                        },
                        "cache": {},
                        "timings": {
                            "blocked": null, "dns": 5.0, "connect": null,
                            "send": 1.0, "wait": 500.0, "receive": 10.0, "ssl": null
                        },
                        "_time": 516.0
                    }
                ]
            }
        }"#,
        )
        .unwrap()
    }

    #[test]
    fn test_parse_har() {
        let har = sample_har();
        assert_eq!(har.log.version, "1.2");
        assert_eq!(har.log.entries.len(), 4);
        assert_eq!(har.log.pages.len(), 1);
    }

    #[test]
    fn test_analyze_stats() {
        let har = sample_har();
        let stats = analyze(&har);

        assert_eq!(stats.total_entries, 4);
        assert_eq!(stats.total_pages, 1);
        assert_eq!(stats.total_transfer_size, 11580); // headersSize + bodySize for all 4 entries
                                                      // Wait: 200+500=700, 180+300=480, 150+10000=10150, 150+100=250. Total=700+480+10150+250=11580
                                                      // Hmm, let me recalculate
                                                      // Entry 1: 200+500=700
                                                      // Entry 2: 180+300=480
                                                      // Entry 3: 150+10000=10150
                                                      // Entry 4: 150+100=250
                                                      // Total: 700+480+10150+250 = 11580
                                                      // But wait, headers_size + body_size for each entry...
                                                      // Entry 1: headers_size=200, body_size=500 => 700
                                                      // Entry 2: headers_size=180, body_size=300 => 480
                                                      // Entry 3: headers_size=150, body_size=10000 => 10150
                                                      // Entry 4: headers_size=150, body_size=100 => 250
                                                      // Total: 11580
        assert_eq!(stats.total_transfer_size, 11580);

        // Status codes
        assert_eq!(stats.status_code_distribution.get(&200), Some(&2)); // 2 GETs
        assert_eq!(stats.status_code_distribution.get(&201), Some(&1));
        assert_eq!(stats.status_code_distribution.get(&500), Some(&1));

        // Content types
        assert_eq!(
            stats.content_type_distribution.get("application/json"),
            Some(&3)
        );
        assert_eq!(stats.content_type_distribution.get("text/css"), Some(&1));

        // Methods
        assert_eq!(stats.method_distribution.get("GET"), Some(&3));
        assert_eq!(stats.method_distribution.get("POST"), Some(&1));

        // Cache
        assert_eq!(stats.cache_hits, 1);
        assert_eq!(stats.cache_misses, 3);

        // Slowest
        assert_eq!(
            stats.slowest_entries[0].url,
            "https://api.example.com/error"
        );
        assert_eq!(stats.slowest_entries[0].total_time_ms, 516.0);
    }

    #[test]
    fn test_domain_stats() {
        let har = sample_har();
        let stats = analyze(&har);

        assert!(stats.per_domain_stats.contains_key("example.com"));
        assert!(stats.per_domain_stats.contains_key("cdn.example.com"));
        assert!(stats.per_domain_stats.contains_key("api.example.com"));

        assert_eq!(
            stats.per_domain_stats.get("example.com").unwrap().requests,
            2
        );
        assert_eq!(
            stats
                .per_domain_stats
                .get("api.example.com")
                .unwrap()
                .errors,
            1
        );
    }

    #[test]
    fn test_page_load_times() {
        let har = sample_har();
        let stats = analyze(&har);

        assert_eq!(stats.page_load_times.len(), 1);
        assert_eq!(stats.page_load_times[0].page_id, "page_1");
        assert_eq!(stats.page_load_times[0].entries, 4);
        assert_eq!(stats.page_load_times[0].on_content_load, Some(100.0));
        assert_eq!(stats.page_load_times[0].on_load, Some(300.0));
    }

    #[test]
    fn test_filter_entries() {
        let har = sample_har();

        // Filter by status
        let filtered = filter_entries(&har.log.entries, Some(400), None, None, None, None, None);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].response.status, 500);

        // Filter by URL pattern
        let filtered = filter_entries(&har.log.entries, None, None, Some("api"), None, None, None);
        assert_eq!(filtered.len(), 3);

        // Filter by min time
        let filtered = filter_entries(&har.log.entries, None, None, None, None, Some(200.0), None);
        assert_eq!(filtered.len(), 1); // only the error at 500ms wait
    }

    #[test]
    fn test_diff_har() {
        let har1 = sample_har();
        let mut har2 = har1.clone();
        // Add a new entry
        let new_entry = har1.log.entries[0].clone();
        har2.log.entries.push(new_entry);

        let diff = diff_har(&har1, &har2);
        assert_eq!(diff.added_entries.len(), 0);
        assert_eq!(diff.same_entries, 5); // All 5 entries matched (one is duplicate by URL+method)
        assert_eq!(diff.request_count_change, 1);
    }

    #[test]
    fn test_check_thresholds() {
        let har = sample_har();
        let stats = analyze(&har);

        // No violations
        let violations = check_thresholds(&stats, Some(10000.0), Some(100), Some(10), Some(500000));
        assert!(violations.is_empty());

        // Trigger violations
        let violations = check_thresholds(&stats, Some(100.0), None, None, None);
        assert!(!violations.is_empty());
    }

    #[test]
    fn test_text_report() {
        let har = sample_har();
        let stats = analyze(&har);
        let report = format_text_report(&stats, &har);
        assert!(report.contains("HAR Analysis Report"));
        assert!(report.contains("Total requests:"));
        assert!(report.contains("example.com"));
    }

    #[test]
    fn test_json_report() {
        let har = sample_har();
        let stats = analyze(&har);
        let report = format_json_report(&stats, None);
        assert!(report.contains("total_entries"));
        assert!(report.contains("total_time_ms"));
    }

    #[test]
    fn test_markdown_report() {
        let har = sample_har();
        let stats = analyze(&har);
        let report = format_markdown_report(&stats, &har);
        assert!(report.contains("# HAR Analysis Report"));
        assert!(report.contains("| Total Requests |"));
    }

    #[test]
    fn test_suggest_filename() {
        let name = suggest_filename("test.har", "report");
        assert_eq!(name, "test-report.md");
    }

    #[test]
    fn test_extract_domain() {
        assert_eq!(
            extract_domain("https://example.com/path"),
            Some("example.com".to_string())
        );
        assert_eq!(
            extract_domain("http://cdn.example.com:8080/"),
            Some("cdn.example.com:8080".to_string())
        );
        assert_eq!(extract_domain("not-a-url"), None);
    }

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(500), "500B");
        assert_eq!(format_size(2048), "2.0KB");
        assert_eq!(format_size(1048576), "1.0MB");
    }

    #[test]
    fn test_parse_error() {
        let result = parse_har("not valid json");
        assert!(result.is_err());
    }
}
