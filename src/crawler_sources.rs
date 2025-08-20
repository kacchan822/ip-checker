use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Crawler IP source configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlerIpSource {
    pub name: String,
    pub url: String,
    pub description: String,
    pub format: String,
}

/// Static crawler IP source configuration (for constants)
#[derive(Debug, Clone)]
pub struct StaticCrawlerIpSource {
    pub name: &'static str,
    pub url: &'static str,
    pub description: &'static str,
    pub format: &'static str,
}

impl From<&StaticCrawlerIpSource> for CrawlerIpSource {
    fn from(static_source: &StaticCrawlerIpSource) -> Self {
        CrawlerIpSource {
            name: static_source.name.to_string(),
            url: static_source.url.to_string(),
            description: static_source.description.to_string(),
            format: static_source.format.to_string(),
        }
    }
}

/// List of crawler IP sources based on the Python implementation
const CRAWLER_IP_SRC_LIST: &[StaticCrawlerIpSource] = &[
    StaticCrawlerIpSource {
        name: "Googlebot IP Ranges",
        url: "https://developers.google.com/search/apis/ipranges/googlebot.json",
        description: "Google 製品で使用される一般的なクローラー（Googlebot など）。自動クロールでは常に robots.txt ルールに従います。",
        format: "JSON",
    },
    StaticCrawlerIpSource {
        name: "Googlebot Special Crawlers IP Ranges",
        url: "https://developers.google.com/static/search/apis/ipranges/special-crawlers.json",
        description: "クロール対象のサイトと Google プロダクトの間でクロール プロセスに関する合意がある Google プロダクトに対して特定の機能を実行するクローラー（AdsBot など）。こうしたクローラーは robots.txt ルールに従う場合と従わない場合があります。",
        format: "JSON",
    },
    StaticCrawlerIpSource {
        name: "Googlebot User Triggered Fetchers IP Ranges",
        url: "https://developers.google.com/static/search/apis/ipranges/user-triggered-fetchers.json",
        description: "エンドユーザーがフェッチをトリガーする、ツールおよびサービスの機能です。",
        format: "JSON",
    },
    StaticCrawlerIpSource {
        name: "Googlebot User Triggered Fetchers IP Ranges (Google)",
        url: "https://developers.google.com/static/search/apis/ipranges/user-triggered-fetchers-google.json",
        description: "エンドユーザーがフェッチをトリガーする、ツールおよびサービスの機能です。",
        format: "JSON",
    },
];

/// Default additional crawler sources (can be overridden by JSON file)
const DEFAULT_ADDITIONAL_SOURCES: &[StaticCrawlerIpSource] = &[
    StaticCrawlerIpSource {
        name: "Bingbot IP Ranges",
        url: "https://www.bing.com/toolbox/bingbot.json",
        description: "Microsoft Bing search engine crawler IP ranges",
        format: "JSON",
    },
    // Note: These URLs are examples and may not be actual endpoints
    // Real implementation would need to verify actual API endpoints
];

/// Load additional crawler sources from JSON file
pub fn load_additional_sources_from_file<P: AsRef<Path>>(
    path: P,
) -> Result<Vec<CrawlerIpSource>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let sources: Vec<CrawlerIpSource> = serde_json::from_str(&content)?;
    Ok(sources)
}

/// Generate a sample JSON file for additional crawler sources
pub fn generate_sample_config_file<P: AsRef<Path>>(
    path: P,
) -> Result<(), Box<dyn std::error::Error>> {
    let sample_sources = vec![
        CrawlerIpSource {
            name: "Example Bot".to_string(),
            url: "https://example.com/bot-ips.json".to_string(),
            description: "Example crawler IP ranges - customize this entry".to_string(),
            format: "JSON".to_string(),
        },
        CrawlerIpSource {
            name: "Another Bot".to_string(),
            url: "https://another-example.com/crawler-ranges.json".to_string(),
            description: "Another example crawler - add more as needed".to_string(),
            format: "JSON".to_string(),
        },
    ];

    let json = serde_json::to_string_pretty(&sample_sources)?;
    fs::write(path, json)?;
    Ok(())
}

/// Get all configured crawler IP sources (built-in + additional from file)
pub fn get_all_crawler_sources() -> Vec<CrawlerIpSource> {
    let mut sources = Vec::new();

    // Add built-in sources
    for static_source in CRAWLER_IP_SRC_LIST {
        sources.push(CrawlerIpSource::from(static_source));
    }

    // Try to load additional sources from file
    const ADDITIONAL_SOURCES_FILE: &str = "additional_crawler_sources.json";
    match load_additional_sources_from_file(ADDITIONAL_SOURCES_FILE) {
        Ok(additional_sources) => {
            sources.extend(additional_sources);
        }
        Err(_) => {
            // If file doesn't exist or has errors, use default additional sources
            for static_source in DEFAULT_ADDITIONAL_SOURCES {
                sources.push(CrawlerIpSource::from(static_source));
            }
        }
    }

    sources
}

/// Get crawler sources by name (case-insensitive partial match)
pub fn get_crawler_sources_by_name(name_filter: &str) -> Vec<CrawlerIpSource> {
    let filter_lower = name_filter.to_lowercase();
    get_all_crawler_sources()
        .into_iter()
        .filter(|source| source.name.to_lowercase().contains(&filter_lower))
        .collect()
}

/// Print crawler source information
pub fn print_crawler_sources(sources: &[CrawlerIpSource], verbose: bool) {
    for (index, source) in sources.iter().enumerate() {
        println!("{}. {}", index + 1, source.name);
        if verbose {
            println!("   URL: {}", source.url);
            println!("   Format: {}", source.format);
            println!("   Description: {}", source.description);
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crawler_sources_list_not_empty() {
        assert!(!CRAWLER_IP_SRC_LIST.is_empty());
        assert!(CRAWLER_IP_SRC_LIST.len() >= 4);
    }

    #[test]
    fn test_get_all_crawler_sources() {
        let sources = get_all_crawler_sources();
        assert!(!sources.is_empty());
        assert!(sources.len() >= CRAWLER_IP_SRC_LIST.len());
    }

    #[test]
    fn test_get_crawler_sources_by_name() {
        let googlebot_sources = get_crawler_sources_by_name("googlebot");
        assert!(!googlebot_sources.is_empty());

        let all_sources = get_crawler_sources_by_name("IP");
        assert!(all_sources.len() >= googlebot_sources.len());

        let no_sources = get_crawler_sources_by_name("nonexistent");
        assert!(no_sources.is_empty());
    }

    #[test]
    fn test_crawler_source_structure() {
        for source in CRAWLER_IP_SRC_LIST {
            assert!(!source.name.is_empty());
            assert!(!source.url.is_empty());
            assert!(!source.description.is_empty());
            assert!(!source.format.is_empty());
            assert!(source.url.starts_with("https://"));
        }
    }
}
