use crate::genome::NeatGenome;
use rand::{Rng, rngs::ThreadRng};
use rayon::prelude::*;

pub fn mutate(population: &mut Vec<NeatGenome>) {
    population.par_iter_mut().for_each(|genome| {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..3) {
            0 => add_node_mutations(genome, &mut rng),
            1 => add_connection_mutations(genome, &mut rng),
            2 => weight_mutations(genome, &mut rng),
            _ => unreachable!(),
        }
    });
}

pub enum MutationType {
    AddNode,
    AddConnection,
    Weight,
}

fn add_node_mutations(individual: &mut NeatGenome, rng: &mut ThreadRng) {
    // Select a random connection via index
    let connection_idx = rng.gen_range(0..individual.connections.len());
    individual.split_connection(connection_idx);
}

fn add_connection_mutations(individual: &mut NeatGenome, rng: &mut ThreadRng) {
    // Select a random node via index
    let node_idx = rng.gen_range(0..individual.nodes.len());
    let node = &individual.nodes[node_idx];

    // Select a random node via index
    let other_node_idx = rng.gen_range(0..individual.nodes.len());
    let other_node = &individual.nodes[other_node_idx];

    // Check if the connection is already present
    if individual.connections.iter().any(|c| c.from_node == node_idx && c.to_node == other_node_idx) {
        // Do nothing
    } else {
        // Add the connection
        individual.add_connection(node_idx, other_node_idx);
    }
}

fn weight_mutations(individual: &mut NeatGenome, rng: &mut ThreadRng) {
    // Select a random connection via index
    let connection_idx = rng.gen_range(0..individual.connections.len());
    let connection = &mut individual.connections[connection_idx];

    // Mutate the weight
    connection.weight += rng.gen_range(-0.5..0.5);
}

