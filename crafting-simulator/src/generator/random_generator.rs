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

    pub fn generate(&self) -> Vec<&'static str> {
        let choices = vec![
            vec!["Basic Synthesis"],
            vec!["Careful Synthesis"],
            vec!["Prudent Synthesis"],
            vec!["Groundwork"],
            vec!["Veneration"],
            vec!["Basic Touch"],
            vec!["Prudent Touch"],
            vec!["Preparatory Touch"],
            vec!["Innovation"],
            vec!["Great Strides"],
            vec!["Byregot's Blessing"],
            // vec!["Observe"],
            // vec!["Focused Synthesis"],
            // vec!["Observe", "Focused Synthesis"],
            // vec!["Focused Touch"],
            // vec!["Observe", "Focused Touch"],
            // vec!["Standard Touch"],
            // vec!["Advanced Touch"],
            vec!["Basic Touch", "Standard Touch"],
            vec!["Basic Touch", "Standard Touch", "Advanced Touch"],
            vec!["Muscle Memory"],
            vec!["Manipulation"],
            vec!["Waste Not"],
            vec!["Waste Not II"],
            // vec!["Master's Mend"],
        ];
        // TODO: extract range and make it seedable?
        let rng = &mut thread_rng();
        let length = rng.gen_range(self.min_length..=self.max_length);
        (0..length)
            .map(|_| choices.choose(rng).unwrap().clone())
            .flatten()
            .collect()
    }
}
