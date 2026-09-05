use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoMetadata {
    pub platform: String,
    pub title: Option<String>,
    pub author: Option<String>,
    pub media_stream_url: Option<String>,
    pub spoken_transcript: String,
    pub original_url: String,
}

pub struct VideoParser;

impl VideoParser {
    pub fn detect_platform(url: &str) -> String {
        let lower = url.to_lowercase();
        if lower.contains("facebook.com") || lower.contains("fb.watch") || lower.contains("fb.com") {
            "Facebook Reels".to_string()
        } else if lower.contains("tiktok.com") {
            "TikTok".to_string()
        } else if lower.contains("youtube.com/shorts") || lower.contains("youtu.be") {
            "YouTube Shorts".to_string()
        } else if lower.contains("instagram.com/reel") {
            "Instagram Reels".to_string()
        } else if lower.contains("x.com") || lower.contains("twitter.com") {
            "X Video".to_string()
        } else {
            "Short Video".to_string()
        }
    }

    pub fn is_video_url(url: &str) -> bool {
        let lower = url.trim().to_lowercase();
        lower.contains("tiktok.com")
            || lower.contains("youtube.com/shorts")
            || lower.contains("youtu.be")
            || lower.contains("instagram.com/reel")
            || lower.contains("facebook.com/reel")
            || lower.contains("fb.watch")
            || lower.contains("facebook.com/watch")
            || lower.contains("fb.com/reel")
            || ((lower.contains("x.com") || lower.contains("twitter.com")) && lower.contains("/status"))
    }

    fn url_encode(s: &str) -> String {
        s.chars().map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            _ => format!("%{:02X}", c as u32),
        }).collect()
    }

    /// Extracts video metadata, media stream source, and post captions using a multi-tiered server scraper.
    pub async fn extract_metadata(url: &str) -> VideoMetadata {
        let mut canonical_url = url.to_string();
        let mut title: Option<String> = None;
        let mut author: Option<String> = None;
        let mut description: Option<String> = None;
        let mut media_stream_url: Option<String> = None;

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(6))
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .redirect(reqwest::redirect::Policy::limited(6))
            .build();

        if let Ok(client) = client {
            // Tier 0: Follow redirects to obtain canonical target URL
            if let Ok(head_resp) = client.get(url).send().await {
                canonical_url = head_resp.url().to_string();
            }

            // Tier 1: Platform-Specific oEmbed API Endpoints
            if canonical_url.contains("tiktok.com") {
                let tiktok_oembed = format!("https://www.tiktok.com/oembed?url={}", Self::url_encode(&canonical_url));
                if let Ok(resp) = client.get(&tiktok_oembed).send().await {
                    if resp.status().is_success() {
                        if let Ok(json) = resp.json::<serde_json::Value>().await {
                            if let Some(t) = json.get("title").and_then(|v| v.as_str()).filter(|s| !s.is_empty()) {
                                title = Some(t.to_string());
                            }
                            if let Some(a) = json.get("author_name").and_then(|v| v.as_str()).filter(|s| !s.is_empty()) {
                                author = Some(a.to_string());
                            }
                        }
                    }
                }
            } else if canonical_url.contains("youtube.com") || canonical_url.contains("youtu.be") {
                let yt_oembed = format!("https://www.youtube.com/oembed?url={}&format=json", Self::url_encode(&canonical_url));
                if let Ok(resp) = client.get(&yt_oembed).send().await {
                    if resp.status().is_success() {
                        if let Ok(json) = resp.json::<serde_json::Value>().await {
                            if let Some(t) = json.get("title").and_then(|v| v.as_str()).filter(|s| !s.is_empty()) {
                                title = Some(t.to_string());
                            }
                            if let Some(a) = json.get("author_name").and_then(|v| v.as_str()).filter(|s| !s.is_empty()) {
                                author = Some(a.to_string());
                            }
                        }
                    }
                }
            }

            // Tier 2: Try noembed.com server-side aggregator if title not found yet
            if title.is_none() {
                let encoded_url = Self::url_encode(&canonical_url);
                let noembed_api = format!("https://noembed.com/embed?url={}", encoded_url);
                if let Ok(resp) = client.get(&noembed_api).send().await {
                    if resp.status().is_success() {
                        if let Ok(json) = resp.json::<serde_json::Value>().await {
                            if let Some(t) = json.get("title").and_then(|v| v.as_str()).filter(|s| !s.is_empty()) {
                                title = Some(t.to_string());
                            }
                            if let Some(a) = json.get("author_name").and_then(|v| v.as_str()).filter(|s| !s.is_empty()) {
                                author = Some(a.to_string());
                            }
                        }
                    }
                }
            }

            // Tier 3: Direct HTML OpenGraph & video stream extraction
            if let Ok(resp) = client.get(&canonical_url).send().await {
                if resp.status().is_success() {
                    if let Ok(html) = resp.text().await {
                        // Extract og:title if not found yet
                        if title.is_none() {
                            if let Some(caps) = html.find("property=\"og:title\" content=\"").or_else(|| html.find("name=\"og:title\" content=\"")) {
                                let sub = &html[caps..];
                                if let Some(start) = sub.find("content=\"") {
                                    let content_sub = &sub[start + 9..];
                                    if let Some(end) = content_sub.find('"') {
                                        title = Some(content_sub[..end].to_string());
                                    }
                                }
                            }
                        }

                        // Extract og:description
                        if let Some(caps) = html.find("property=\"og:description\" content=\"").or_else(|| html.find("name=\"og:description\" content=\"")) {
                            let sub = &html[caps..];
                            if let Some(start) = sub.find("content=\"") {
                                let content_sub = &sub[start + 9..];
                                if let Some(end) = content_sub.find('"') {
                                    description = Some(content_sub[..end].to_string());
                                }
                            }
                        }

                        // Extract direct media stream URL
                        if let Some(caps) = html.find("property=\"og:video\" content=\"").or_else(|| html.find("property=\"og:video:secure_url\" content=\"")) {
                            let sub = &html[caps..];
                            if let Some(start) = sub.find("content=\"") {
                                let content_sub = &sub[start + 9..];
                                if let Some(end) = content_sub.find('"') {
                                    media_stream_url = Some(content_sub[..end].to_string());
                                }
                            }
                        }
                    }
                }
            }
        }

        let platform = Self::detect_platform(&canonical_url);

        let spoken_transcript = match (&title, &description, &author) {
            (Some(t), Some(d), Some(a)) => format!("[Reel Title & Caption on {}]: \"{}\" - \"{}\" (by {})", platform, t, d, a),
            (Some(t), Some(d), None) => format!("[Reel Title & Caption on {}]: \"{}\" - \"{}\"", platform, t, d),
            (Some(t), None, Some(a)) => format!("[Reel Title on {}]: \"{}\" (by {})", platform, t, a),
            (Some(t), None, None) => format!("[Reel Title on {}]: \"{}\"", platform, t),
            _ => {
                let clean_slug = canonical_url.split('?').next().unwrap_or(&canonical_url).split('/').filter(|s| !s.is_empty()).last().unwrap_or("reel");
                format!("[Extracted {} Video Content ({})]", platform, clean_slug)
            }
        };

        VideoMetadata {
            platform,
            title,
            author,
            media_stream_url,
            spoken_transcript,
            original_url: url.to_string(),
        }
    }
}

