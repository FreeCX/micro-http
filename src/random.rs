use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Default, Clone, Copy)]
pub struct Random {
    init: u32,
}

impl Random {
    pub fn new() -> Random {
        // в качестве seed будем использовать текущее время
        let since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(Duration::from_secs(42));
        Random { init: since_epoch.as_secs() as u32 }
    }

    // в качестве ГПСЧ нам хватит xor shift
    pub fn generate(&mut self) -> u32 {
        self.init ^= self.init << 13;
        self.init ^= self.init >> 17;
        self.init ^= self.init << 5;
        self.init
    }

    pub fn in_range(&mut self, min: i32, max: i32) -> i32 {
        let value = self.generate() % (max - min).unsigned_abs();
        value as i32 + min
    }
}
