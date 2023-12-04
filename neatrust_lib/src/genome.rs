use rand::Rng;

#[derive(Clone, Copy)]
struct ConnectionGene {
    pub from_node: usize,
    pub to_node: usize,
    pub weight: f64,
    pub enabled: bool,
    pub innovation_number: usize,
}

#[derive(Clone, Copy)]
struct NodeGene {
    pub node_type: NodeType,
}

#[derive(Clone, Copy)]
enum NodeType {
    Input,
    Output,
    Hidden,
}


pub struct NeatGenome {
    pub nodes: Vec<NodeGene>,
    pub connections: Vec<ConnectionGene>,
    pub innovation_number: usize,
    pub species: Option<usize>,
    pub fitness: Option<f64>,
}

impl NeatGenome {
    fn new_random<R: Rng>(rng: &mut R, input_nodes: usize, output_nodes: usize) -> Self {
        // Initialize nodes with input and output nodes
        let mut nodes = Vec::with_capacity(input_nodes + output_nodes);
        for i in 0..input_nodes {
            nodes.push(NodeGene { node_type: NodeType::Input });
        }
        for i in 0..output_nodes {
            nodes.push(NodeGene { node_type: NodeType::Output });
        }

        let mut innovation_number: usize = 0;
        // Initialize connections with random weights
        let mut connections = Vec::new();
        for from_node in 0..input_nodes {
            for to_node in input_nodes..input_nodes + output_nodes {
                // Randomly enable or disable connections
                let enabled = rng.gen::<f64>() < 0.5;
                // Randomly assign weights
                let weight = rng.gen::<f64>() * 2.0 - 1.0; // Range [-1.0, 1.0]
                connections.push(ConnectionGene { from_node, to_node, weight, enabled, innovation_number });
                innovation_number += 1;
            }
        }

        NeatGenome { nodes, connections, innovation_number, fitness: None, species: None }
    }

    pub fn new(input_nodes: usize, output_nodes: usize) -> Self {
        let mut rng = rand::thread_rng();
        Self::new_random(&mut rng, input_nodes, output_nodes)
    }

    pub fn add_connection(mut self, from_node: usize, to_node: usize) -> Self {
        let mut rng = rand::thread_rng();
        let new_connection = ConnectionGene {
            from_node,
            to_node,
            weight: rng.gen::<f64>() * 2.0 - 1.0,
            enabled: true,
            innovation_number: self.innovation_number,
        };
        self.connections.push(new_connection);
        self.innovation_number += 1;
        self
    }

    pub fn split_connection(mut self, connection_idx: usize) -> Self {
        let new_node_id = self.nodes.len();
        let new_node = NodeGene { node_type: NodeType::Hidden };
        self.nodes.push(new_node);

        let connection = self.connections[connection_idx];
        let new_connection_1 = ConnectionGene {
            from_node: connection.from_node,
            to_node: new_node_id,
            weight: 1.0,
            enabled: true,
            innovation_number: self.innovation_number,
        };
        let new_connection_2 = ConnectionGene {
            from_node: new_node_id,
            to_node: connection.to_node,
            weight: connection.weight,
            enabled: true,
            innovation_number: self.innovation_number + 1,
        };
        self.connections[connection_idx] = new_connection_1;
        self.connections.push(new_connection_2);
        self.connections[connection_idx].enabled = false;
        self.innovation_number += 2;

        self
    }

    pub fn get_fitness(&self) -> Option<f64> {
        self.fitness
    }

    pub fn calculate_genome_distance(&self, other: &Self) -> f64 {
        let mut excess_genes = 0;
        let mut disjoint_genes = 0;
        let mut weight_difference = 0.0;
        let mut matching_genes = 0;

        let mut self_idx = 0;
        let mut other_idx = 0;
        while self_idx < self.connections.len() && other_idx < other.connections.len() {
            let self_innovation_number = self.connections[self_idx].innovation_number;
            let other_innovation_number = other.connections[other_idx].innovation_number;
            if self_innovation_number == other_innovation_number {
                matching_genes += 1;
                weight_difference += (self.connections[self_idx].weight - other.connections[other_idx].weight).abs();
                self_idx += 1;
                other_idx += 1;
            } else if self_innovation_number < other_innovation_number {
                disjoint_genes += 1;
                self_idx += 1;
            } else {
                disjoint_genes += 1;
                other_idx += 1;
            }
        }
        if self_idx < self.connections.len() {
            excess_genes += self.connections.len() - self_idx;
        }
        if other_idx < other.connections.len() {
            excess_genes += other.connections.len() - other_idx;
        }

        let n = matching_genes as f64;
        let mut c1 = 1.0;
        let mut c2 = 1.0;
        let mut c3 = 0.4;
        if self.connections.len() > 20 && other.connections.len() > 20 {
            c1 = 1.0;
            c2 = 1.0;
            c3 = 0.4;
        }

        let mut distance = (c1 * excess_genes as f64) / n + (c2 * disjoint_genes as f64) / n + c3 * weight_difference / n;
        if distance.is_nan() {
            distance = 100.0;
        }
        distance
    }
}

