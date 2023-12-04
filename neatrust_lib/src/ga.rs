use crate::algorithm::Algorithm;
use crate::algorithm::AlgorithmOptions;
use crate::neat::NEAT;

// The genetic algorithm implementation.
// Provides the training loop for the hill climber and the NEAT algorithm.
// -----------------------------------------------------------------------------


pub fn genetic_algorithm(options: AlgorithmOptions) {
    let max_generations = options.max_generations;

    // 1. Initialize population and algorithm
    let mut algorithm: NEAT = NEAT::new(options);

    // 2. Loop until solution is found
    while algorithm.get_generation() < max_generations {
        
        // 3. Evaluate fitness
        algorithm.evaluate();

        // 4. Selection. For hill climber, this is a no-op.
        // For NEAT, this is speciation and selection.
        algorithm.select();

        // 5. Cross-over. For hill climber, this is a no-op.
        // For NEAT, this is mating.
        algorithm.crossover();

        // 6. Mutation. For hill climber, this is a search of the neighborhood.
        // For NEAT, this is mutation of the genome.
        algorithm.mutate();

        // 7. Increment generation
        algorithm.increment_generation();
    }
}