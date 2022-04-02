pub struct CharSampler {
    chars: String
}

impl CharSampler {
    pub fn new(chars: String) -> Self {
        Self { chars }
    }

    pub fn sample(&self, level: f32) -> char {
        let pos = level * self.chars.chars().count() as f32;
        let index = pos as usize;

        self.chars.chars().nth(index).unwrap()
    }
}