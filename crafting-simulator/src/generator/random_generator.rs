use rand::{seq::SliceRandom, thread_rng, Rng};

pub struct RandomGenerator {
    pub min_length: u8,
    pub max_length: u8,
}

impl RandomGenerator {
    pub fn from_lengths(min_length: u8, max_length: u8) -> RandomGenerator {
        RandomGenerator {
            min_length,
            max_length,
        }
    }

    pub fn generate(&mut self) -> Vec<&'static str> {
        let choices = vec![
            "Basic Synthesis",
            "Careful Synthesis",
            "Prudent Synthesis",
            "Groundwork",
            "Veneration",
            "Basic Touch",
            "Prudent Touch",
            "Preparatory Touch",
            "Innovation",
            "Great Strides",
            "Byregot's Blessing",
            // "Observe",
            // "Focused Synthesis",
            "Focused Touch",
            // "Standard Touch",
            // "Advanced Touch",
            "Muscle Memory",
            "Manipulation",
            // "Waste Not",
            "Waste Not II",
            // "Master's Mend",
        ];
        let rng = &mut thread_rng();
        let length = rng.gen_range(self.min_length..=self.max_length);
        (0..length).map(|_| *choices.choose(rng).unwrap()).collect()
    }
}
