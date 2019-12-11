use std::error::Error;
use crate::modules::graph;

pub fn run_part1(input: &str) -> Result<String, Box<dyn Error>> {
    let orbits = graph::build_graph_from_orbit_input(input)?;
    
    let (depths, _) = orbits.bfs("COM");
    let mut total = 0;

    for (_, depth) in &depths {
        total += depth;
    }

    Ok(format!("Total number of orbits: {}", total))
}

pub fn run_part2(input: &str) -> Result<String, Box<dyn Error>> {
    let orbits = graph::build_graph_from_orbit_input(input)?;

    let (_, paths) = orbits.bfs("SAN");
    let mut transfers = 0;
    let mut target = paths.get("YOU").unwrap();

    loop {
        target = paths.get(target).unwrap();
        if target == "SAN" {
            break;
        }

        transfers += 1;
    }

    Ok(format!("Total number of orbital transfers needed: {}", transfers))
}
