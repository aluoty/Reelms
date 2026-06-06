use std::time::{SystemTime, UNIX_EPOCH};

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

#[derive(Clone, Debug, Default)]
pub struct RestManager {
    next_rest_at_ms: i64,
}

impl RestManager {
    pub fn can_rest_now(&self) -> bool {
        now_ms() >= self.next_rest_at_ms
    }

    pub fn trigger_cooldown_ms(&mut self, cooldown_ms: i64) {
        self.next_rest_at_ms = now_ms() + cooldown_ms.max(0);
    }

    pub fn seconds_remaining(&self) -> i64 {
        ((self.next_rest_at_ms - now_ms()).max(0)) / 1000
    }

    pub fn set_next_rest_at_ms(&mut self, value: i64) {
        self.next_rest_at_ms = value;
    }

    pub fn next_rest_at_ms(&self) -> i64 {
        self.next_rest_at_ms
    }
}
