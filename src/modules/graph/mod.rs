use std::fs;
use std::io;
use std::collections::HashMap;

pub struct Graph {
    nodes: HashMap<String, Node>,
}

impl Graph {
    pub fn bfs(self, source_id: &str) -> HashMap<String, i32> {
        let mut frontier: Vec<String> = vec![source_id.to_string()];
        let mut depths: HashMap<String, i32> = HashMap::new();
        let mut level = 1;
        depths.insert(source_id.to_string(), 0);
        
        while !frontier.is_empty() {
            let mut next: Vec<String> = Vec::new();
            
            for node_id in frontier {
                if let Some(node) = self.nodes.get(&node_id) {
                    
                    for neighbor in node.neighbors.iter() {
                        if !depths.contains_key(&neighbor.to_string()) {
                            depths.insert(neighbor.to_string(), level);
                            next.push(neighbor.to_string());
                        }
                    }
                }
            }

            frontier = next;
            level += 1;
        }

        depths
    }
}

struct Node {
    neighbors: Vec<String>,
}

pub fn build_graph_from_orbit_input(input: &str) -> Result<Graph, io::Error> {
    let contents = fs::read_to_string(input)?;
    let orbits = contents.split("\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty());
    let mut node_map: HashMap<String, Node> = HashMap::new();


    for orbit in orbits {
        let objects: Vec<&str> = orbit.split(")")
            .collect();
        
        match node_map.get_mut(&objects[0].to_string()) {
            Some(ref mut parent) => {
                parent.neighbors.push(objects[1].to_string());
            }
            None => { 
                let parent = Node {
                    neighbors: vec![objects[1].to_string()],
                };
                node_map.insert(objects[0].to_string(), parent); 
                continue;
            }
        }
    }

    Ok(Graph {
        nodes: node_map,
    })
} 

