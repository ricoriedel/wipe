/// Used to get a character with a given brightness.
pub struct CharSampler {
    chars: String
}

impl CharSampler {
    /// # Arguments
    /// * `chars`: The characters ordered by brightness.
    pub fn new(chars: String) -> Self {
        Self { chars }
    }

    /// Gets a character with the given brightness.
    /// # Arguments
    /// * `level`: `0 <= level` and `level < 1`
    pub fn sample(&self, level: f32) -> char {
        let pos = level * self.chars.chars().count() as f32;
        let index = pos as usize;

        self.chars.chars().nth(index).unwrap()
    }
}