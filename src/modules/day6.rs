use std::error::Error;
use crate::modules::graph;

pub fn run(input: &str) -> Result<String, Box<dyn Error>> {
    let orbits = graph::build_graph_from_orbit_input(input)?;
    
    let depths = orbits.bfs("COM");
    let mut total = 0;

    for (_, depth) in &depths {
        total += depth;
    }

    Ok(format!("Total number of orbits: {}", total))
}
