pub trait Algorithm {
    fn evaluate(&mut self);
    fn select(&mut self);
    fn crossover(&mut self);
    fn mutate(&mut self);
    fn increment_generation(&mut self);
    fn get_generation(&self) -> u32;
}

#[derive(Default)]
pub struct AlgorithmOptions {
    pub population_size: u32,
    pub max_generations: u32,
    pub minimum_distance: f64,
}