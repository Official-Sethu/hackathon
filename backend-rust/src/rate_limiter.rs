use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Thread-safe in-memory sliding window rate limiter
pub struct RateLimiter {
    requests: Mutex<HashMap<String, Vec<Instant>>>,
    max_requests: usize,
    window: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window_secs: u64) -> Self {
        Self {
            requests: Mutex::new(HashMap::new()),
            max_requests,
            window: Duration::from_secs(window_secs),
        }
    }

    /// Checks if a client has exceeded their allowed quota
    pub fn check(&self, client_id: &str) -> Result<(), (u64, String)> {
        let mut map = self.requests.lock().map_err(|_| (5, "Lock acquisition error".to_string()))?;
        let now = Instant::now();

        let timestamps = map.entry(client_id.to_string()).or_default();
        // Prune entries outside the time window
        timestamps.retain(|&t| now.duration_since(t) < self.window);

        if timestamps.len() >= self.max_requests {
            let oldest = timestamps[0];
            let elapsed = now.duration_since(oldest);
            let retry_after = if elapsed < self.window {
                (self.window - elapsed).as_secs() + 1
            } else {
                1
            };
            return Err((
                retry_after,
                format!(
                    "Rate limit exceeded: maximum {} requests per {}s. Retry in {}s.",
                    self.max_requests,
                    self.window.as_secs(),
                    retry_after
                ),
            ));
        }

        timestamps.push(now);
        Ok(())
    }
}
