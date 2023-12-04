use crate::neat::NEAT;


pub fn speciation_neat(algorithm: &mut NEAT) {
    let mut representatives = Vec::new();
    // Genome Loop
    // Take next genome g from population
    // If not all genomes in G have been checked, go to Genome loop
    for genome in algorithm.population.mut_iter() {
        // The Species Loop:
        // get next species s from S
        for s in (0..algorithm.species.len()) {
            // If g is compatible with s, add g to s
            // Else, go to Species Loop
        }
    }
    // Else STOP
}