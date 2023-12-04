use rand::{thread_rng, Rng};

use super::RandomGenerator;

pub struct RandomFlip {
    new_step_generator: RandomGenerator,
}

impl RandomFlip {
    pub fn new() -> RandomFlip {
        RandomFlip {
            new_step_generator: RandomGenerator {
                min_length: 1,
                max_length: 1,
            },
        }
    }

    pub fn apply<'a>(&self, input: &[&'a str]) -> Vec<&'a str> {
        let mut result = input.to_vec();

        let rng = &mut thread_rng();

        let new_items = self.new_step_generator.generate();
        let index = rng.gen_range(0..result.len() - 1);

        result.splice(index..index + 1, new_items);

        result
    }
}

pub struct RandomRemove {}

impl RandomRemove {
    pub fn apply<'a>(&self, input: &[&'a str]) -> Vec<&'a str> {
        let mut result = input.to_vec();

        let rng = &mut thread_rng();

        let index = rng.gen_range(0..result.len() - 1);

        result.splice(index..index + 1, []);

        result
    }
}
