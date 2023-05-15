use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Random {
    init: u32,
}

impl Default for Random {
    fn default() -> Self {
        Self::new()
    }
}

impl Random {
    pub fn new() -> Random {
        let start = SystemTime::now();
        let since_epoch = start.duration_since(UNIX_EPOCH).unwrap_or(Duration::from_secs(42));
        Random { init: since_epoch.as_secs() as u32 }
    }

    pub fn generate(&mut self) -> u32 {
        self.init ^= self.init << 13;
        self.init ^= self.init >> 17;
        self.init ^= self.init << 5;
        self.init
    }

    pub fn in_range(&mut self, min: u32, max: u32) -> u32 {
        self.generate() % (max - min) + min
    }
}
