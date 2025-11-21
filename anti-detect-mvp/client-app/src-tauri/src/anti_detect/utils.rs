/// Utility functions for anti-detection

/// Generate a consistent hash from a string seed
pub fn seed_hash(seed: &str) -> u64 {
    let mut hash: u64 = 5381;
    for byte in seed.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
    }
    hash
}

/// Seeded random number generator
pub struct SeededRng {
    state: u64,
}

impl SeededRng {
    pub fn new(seed: &str) -> Self {
        Self {
            state: seed_hash(seed),
        }
    }
    
    /// Generate next random number between 0.0 and 1.0
    pub fn next(&mut self) -> f64 {
        self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
        ((self.state / 65536) % 32768) as f64 / 32768.0
    }
    
    /// Generate random number in range [min, max]
    pub fn range(&mut self, min: f64, max: f64) -> f64 {
        min + (max - min) * self.next()
    }
    
    /// Generate random integer in range [min, max)
    pub fn range_int(&mut self, min: i32, max: i32) -> i32 {
        min + (self.range(0.0, 1.0) * (max - min) as f64) as i32
    }
}

/// Common screen resolutions
pub fn common_screen_resolutions() -> Vec<(u32, u32)> {
    vec![
        (1920, 1080),
        (1366, 768),
        (1536, 864),
        (1440, 900),
        (2560, 1440),
        (1600, 900),
        (1280, 720),
        (1280, 800),
        (1680, 1050),
        (3840, 2160), // 4K
    ]
}

/// Common user agents
pub fn common_user_agents() -> Vec<String> {
    vec![
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36".to_string(),
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36".to_string(),
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36".to_string(),
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36".to_string(),
    ]
}

/// WebGL vendors and renderers
pub fn webgl_configurations() -> Vec<(String, String)> {
    vec![
        ("Google Inc. (NVIDIA)".to_string(), "ANGLE (NVIDIA GeForce GTX 1060 Direct3D11 vs_5_0 ps_5_0)".to_string()),
        ("Google Inc. (NVIDIA)".to_string(), "ANGLE (NVIDIA GeForce RTX 3060 Direct3D11 vs_5_0 ps_5_0)".to_string()),
        ("Google Inc. (Intel)".to_string(), "ANGLE (Intel(R) UHD Graphics 630 Direct3D11 vs_5_0 ps_5_0)".to_string()),
        ("Google Inc. (AMD)".to_string(), "ANGLE (AMD Radeon RX 580 Series Direct3D11 vs_5_0 ps_5_0)".to_string()),
        ("Intel Inc.".to_string(), "Intel Iris OpenGL Engine".to_string()),
        ("Apple Inc.".to_string(), "Apple M1 GPU".to_string()),
        ("Apple Inc.".to_string(), "Apple M2 GPU".to_string()),
    ]
}

/// Common fonts across platforms
pub fn common_fonts() -> Vec<String> {
    vec![
        "Arial", "Arial Black", "Arial Narrow", "Arial Rounded MT Bold",
        "Bookman Old Style", "Bradley Hand ITC", "Calibri", "Cambria",
        "Cambria Math", "Comic Sans MS", "Consolas", "Constantia",
        "Courier New", "Franklin Gothic Medium", "Garamond", "Georgia",
        "Helvetica", "Impact", "Lucida Console", "Lucida Sans Unicode",
        "Microsoft Sans Serif", "MS Gothic", "MS Sans Serif", "Palatino Linotype",
        "Segoe UI", "Tahoma", "Times New Roman", "Trebuchet MS", "Verdana",
    ].iter().map(|&s| s.to_string()).collect()
}

/// Timezone list
pub fn timezones() -> Vec<(&'static str, i32)> {
    vec![
        ("America/New_York", -300),
        ("America/Los_Angeles", -480),
        ("America/Chicago", -360),
        ("America/Denver", -420),
        ("America/Phoenix", -420),
        ("Europe/London", 0),
        ("Europe/Paris", 60),
        ("Europe/Berlin", 60),
        ("Europe/Moscow", 180),
        ("Asia/Tokyo", 540),
        ("Asia/Shanghai", 480),
        ("Asia/Dubai", 240),
        ("Asia/Kolkata", 330),
        ("Australia/Sydney", 600),
        ("Pacific/Auckland", 720),
    ]
}

/// Language codes
pub fn languages() -> Vec<Vec<&'static str>> {
    vec![
        vec!["en-US", "en"],
        vec!["en-GB", "en"],
        vec!["es-ES", "es"],
        vec!["fr-FR", "fr"],
        vec!["de-DE", "de"],
        vec!["it-IT", "it"],
        vec!["pt-BR", "pt"],
        vec!["ru-RU", "ru"],
        vec!["ja-JP", "ja"],
        vec!["zh-CN", "zh"],
        vec!["ko-KR", "ko"],
        vec!["ar-SA", "ar"],
    ]
}

/// Hardware concurrency options (CPU cores)
pub fn hardware_concurrency_options() -> Vec<u8> {
    vec![2, 4, 6, 8, 12, 16, 24, 32]
}

/// Device memory options (GB)
pub fn device_memory_options() -> Vec<u8> {
    vec![2, 4, 8, 16, 32, 64]
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_seeded_rng() {
        let mut rng1 = SeededRng::new("test-seed");
        let mut rng2 = SeededRng::new("test-seed");
        
        // Same seed should produce same sequence
        for _ in 0..100 {
            assert_eq!(rng1.next(), rng2.next());
        }
    }
    
    #[test]
    fn test_seeded_rng_different_seeds() {
        let mut rng1 = SeededRng::new("seed1");
        let mut rng2 = SeededRng::new("seed2");
        
        // Different seeds should produce different sequences
        let val1 = rng1.next();
        let val2 = rng2.next();
        assert_ne!(val1, val2);
    }
}
