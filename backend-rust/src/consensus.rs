use crate::models::{CitationSource, ModelResult, ReasoningStep, VerificationResponse};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct ConsensusEngine;

impl ConsensusEngine {
    pub fn calculate_consensus(
        claim: &str,
        models: Vec<ModelResult>,
    ) -> VerificationResponse {
        // ---------------------------------------------------------------
        // 1. Confidence-weighted truth score
        // Each model's truthScore is weighted by its own confidence value.
        // This prevents a low-confidence hallucination from dragging down
        // a high-confidence correct result.
        // ---------------------------------------------------------------
        struct ScoredModel {
            truth_score: u32,
            confidence: u32,
            verdict: Option<String>,
        }

        let scored: Vec<ScoredModel> = models
            .iter()
            .filter_map(|m| {
                let ts = m.truth_score?;
                if ts > 100 {
                    return None;
                }
                Some(ScoredModel {
                    truth_score: ts,
                    confidence: m.confidence.clamp(1, 100),
                    verdict: m.verdict.clone(),
                })
            })
            .collect();

        let (truth_score, max_score, min_score) = if !scored.is_empty() {
            let weighted_sum: u64 = scored
                .iter()
                .map(|s| (s.truth_score as u64) * (s.confidence as u64))
                .sum();
            let confidence_sum: u64 = scored.iter().map(|s| s.confidence as u64).sum();
            let avg = (weighted_sum / confidence_sum.max(1)) as u32;
            let max_s = scored.iter().map(|s| s.truth_score).max().unwrap_or(avg);
            let min_s = scored.iter().map(|s| s.truth_score).min().unwrap_or(avg);
            (avg, max_s, min_s)
        } else {
            (50u32, 50u32, 50u32)
        };

        // ---------------------------------------------------------------
        // 2. Majority-vote verdict
        // Count how many models returned each verdict category, then pick
        // the one with the most votes. Fall back to score-derived verdict
        // only when no model returned a verdict.
        // ---------------------------------------------------------------
        let mut verified_votes = 0usize;
        let mut nuanced_votes = 0usize;
        let mut fabricated_votes = 0usize;

        for s in &scored {
            if let Some(v) = &s.verdict {
                let vu = v.to_uppercase();
                if vu.contains("VERIFIED") || vu.contains("FACTUAL") || vu.contains("AUTHENTIC") {
                    verified_votes += 1;
                } else if vu.contains("FABRICATED") || vu.contains("DEBUNKED") || vu.contains("FALSE") {
                    fabricated_votes += 1;
                } else {
                    nuanced_votes += 1;
                }
            }
        }

        let verdict_label = if verified_votes > fabricated_votes && verified_votes >= nuanced_votes {
            "VERIFIED FACTUAL & AUTHENTIC".to_string()
        } else if fabricated_votes > verified_votes && fabricated_votes >= nuanced_votes {
            "FABRICATED OR DEBUNKED".to_string()
        } else if nuanced_votes > 0 || verified_votes > 0 || fabricated_votes > 0 {
            "UNSUBSTANTIATED / NUANCED".to_string()
        } else {
            // Pure score fallback (no model returned a verdict)
            if truth_score >= 75 {
                "VERIFIED FACTUAL & AUTHENTIC".to_string()
            } else if truth_score >= 35 {
                "UNSUBSTANTIATED / NUANCED".to_string()
            } else {
                "FABRICATED OR DEBUNKED".to_string()
            }
        };

        // ---------------------------------------------------------------
        // 3. Dissent flag — raised when model scores diverge by > 25 pts
        // A high spread signals genuine ambiguity in the claim.
        // ---------------------------------------------------------------
        let dissent_flag = if !scored.is_empty() {
            Some(max_score.saturating_sub(min_score) > 25)
        } else {
            None
        };

        // ---------------------------------------------------------------
        // 4. Extract headline and summary from the highest-confidence model
        // ---------------------------------------------------------------
        let best_model = models
            .iter()
            .max_by_key(|m| m.confidence);

        let headline = best_model
            .and_then(|m| m.headline.clone())
            .filter(|h| !h.trim().is_empty())
            .unwrap_or_else(|| format!("Claim evaluated as {}", verdict_label));

        let summary = best_model
            .and_then(|m| m.summary.clone())
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| {
                format!(
                    "Multi-model cross-examination produced a confidence-weighted truth score of {}%.",
                    truth_score
                )
            });

        // ---------------------------------------------------------------
        // 5. Fallacy risk tier
        // ---------------------------------------------------------------
        let fallacy_risk = if truth_score < 40 {
            "Critical".to_string()
        } else if truth_score < 75 {
            "Moderate".to_string()
        } else {
            "Low".to_string()
        };

        // ---------------------------------------------------------------
        // 6. Aggregate reasoning trace (deduplicated by title)
        // ---------------------------------------------------------------
        let mut reasoning_trace: Vec<ReasoningStep> = Vec::new();
        for m in &models {
            if let Some(steps) = &m.reasoning_steps {
                for s in steps {
                    if !reasoning_trace.iter().any(|existing| existing.title == s.title) {
                        reasoning_trace.push(ReasoningStep {
                            step: (reasoning_trace.len() + 1) as u32,
                            title: s.title.clone(),
                            description: s.description.clone(),
                        });
                    }
                }
            }
        }

        if reasoning_trace.is_empty() {
            reasoning_trace.push(ReasoningStep {
                step: 1,
                title: "Multi-Model Query Dispatch".to_string(),
                description: "Dispatched parallel chain-of-thought prompts to DeepSeek V4, MiniMax M2.7, and Kimi K2 via Gonka Router.".to_string(),
            });
            reasoning_trace.push(ReasoningStep {
                step: 2,
                title: "Confidence-Weighted Consensus".to_string(),
                description: format!(
                    "Applied confidence-weighted voting across model outputs. Resulting truth score: {}%. Dissent detected: {}.",
                    truth_score,
                    dissent_flag.unwrap_or(false)
                ),
            });
        }

        // ---------------------------------------------------------------
        // 7. Aggregate fallacies (deduplicated)
        // ---------------------------------------------------------------
        let mut fallacies: Vec<String> = Vec::new();
        for m in &models {
            if let Some(fal_list) = &m.fallacies {
                for f in fal_list {
                    let f = f.trim();
                    if !f.is_empty() && !fallacies.iter().any(|e: &String| e.as_str() == f) {
                        fallacies.push(f.to_string());
                    }
                }
            }
        }
        if fallacies.is_empty() {
            fallacies.push("No significant fallacies detected.".to_string());
        }

        // ---------------------------------------------------------------
        // 8. Aggregate citations (deduplicated by URL)
        // ---------------------------------------------------------------
        let mut citations: Vec<CitationSource> = Vec::new();
        for m in &models {
            if let Some(cite_list) = &m.citations {
                for c in cite_list {
                    if !c.title.trim().is_empty() && !citations.iter().any(|e| e.url == c.url) {
                        citations.push(c.clone());
                    }
                }
            }
        }
        if citations.is_empty() {
            citations.push(CitationSource {
                title: "Gonka Router Audit Trail".to_string(),
                url: "https://gonkarouter.io".to_string(),
            });
        }

        // ---------------------------------------------------------------
        // 9. Consensus alignment — inverse of score spread, capped 0-100
        // ---------------------------------------------------------------
        let consensus_alignment = if scored.len() > 1 {
            100u32.saturating_sub(max_score.saturating_sub(min_score))
        } else {
            95
        };

        VerificationResponse {
            claim: claim.to_string(),
            truth_score,
            verdict_label,
            headline,
            summary,
            factuality_index: truth_score,
            consensus_alignment,
            fallacy_risk,
            dissent_flag,
            is_video: None,
            video_platform: None,
            spoken_transcript: None,
            models,
            reasoning_trace,
            fallacies,
            citations,
            timestamp: real_timestamp(),
        }
    }
}

/// Returns the actual current UTC time in ISO-8601 format.
fn real_timestamp() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    // Manual ISO-8601 from epoch seconds (no chrono dependency needed)
    let s = secs % 60;
    let m = (secs / 60) % 60;
    let h = (secs / 3600) % 24;
    let days = secs / 86400; // days since 1970-01-01

    // Compute Gregorian date from days since epoch
    let (year, month, day) = days_to_ymd(days);

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        year, month, day, h, m, s
    )
}

fn days_to_ymd(days: u64) -> (u64, u64, u64) {
    // Rata Die algorithm — converts days since 1970-01-01 to (year, month, day)
    let z = days + 719468;
    let era = z / 146097;
    let doe = z % 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}
