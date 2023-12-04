use crate::genome::NeatGenome;
use crate::algorithm::{Algorithm, AlgorithmOptions};
use crate::mutation::mutate;


pub struct NEAT {
    pub population: Vec<NeatGenome>,
    pub species: Vec<usize>,
    pub generation: u32,
    pub options: AlgorithmOptions,
}

impl NEAT {
    pub fn new(options: AlgorithmOptions) -> Self {
        let mut population = Vec::with_capacity(options.population_size as usize);
        for _ in 0..options.population_size {
            population.push(NeatGenome::new(1, 1));
        }

        Self {
            population,
            species: vec![0],
            generation: 0,
            options,
        }
    }
}

impl Algorithm for NEAT {
    fn evaluate(&mut self) {
        
    }

    fn select(&mut self) {}

    fn crossover(&mut self) {}

    fn mutate(&mut self) {
        mutate(&mut self.population);
    }

    fn increment_generation(&mut self) {
        self.generation += 1;
    }

    fn get_generation(&self) -> u32 {
        self.generation
    }
}