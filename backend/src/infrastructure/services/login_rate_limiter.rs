use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

#[derive(Clone)]
pub struct LoginRateLimiter {
    inner: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
    max_requests: usize,
    window: Duration,
}

impl LoginRateLimiter {
    pub fn new(max_requests: usize, window_seconds: u64) -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window: Duration::from_secs(window_seconds),
        }
    }

    pub fn allow(&self, key: &str) -> bool {
        let now = Instant::now();
        let mut map = self.inner.lock().expect("rate limiter lock");
        let entries = map.entry(key.to_string()).or_default();

        entries.retain(|timestamp| now.duration_since(*timestamp) <= self.window);

        if entries.len() >= self.max_requests {
            return false;
        }

        entries.push(now);
        true
    }
}
