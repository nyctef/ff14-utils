use rand::{seq::SliceRandom, thread_rng};

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
        let choices = vec!["Basic Synthesis", "Basic Touch"];
        let rng = &mut thread_rng();
        let length = self.min_length; // TODO
        (0..length).map(|_| *choices.choose(rng).unwrap()).collect()
    }
}
