use std::{collections::{hash_map, HashMap}, fs::{self, File}, io::{Error, Write}};
use rand::Rng;

pub struct Graph {
    source_vertices: HashMap<i32, Vec<i32>>,
    destination_vertices: HashMap<i32, Vec<i32>>,
    edge_cost: HashMap<(i32, i32), i32>
}

impl Graph {
    pub fn new() -> Self {
        Self {
            source_vertices: HashMap::new(),
            destination_vertices: HashMap::new(),
            edge_cost: HashMap::new(),
        }
    }

    pub fn new_from_file(filename: String) -> Result<Self, Error> {
        let file_content = fs::read_to_string(filename)?;

        let mut result = Self {
            source_vertices: HashMap::new(),
            destination_vertices: HashMap::new(),
            edge_cost: HashMap::new(),
        };

        let mut _vertex_number: i32;
        let mut _edge_number: i32;

        for (index, line) in file_content.lines().enumerate() {
            let l = line.to_string();
            let mut l = l.split_whitespace();
            if index == 0 {
                _vertex_number = l.next().unwrap().parse().unwrap();
                _edge_number = l.next().unwrap().parse().unwrap();
                for i in 0.._vertex_number {
                    result.add_vertex(i);
                }
                // println!("{} {}", vertex_number, edge_number);
            } else {
                let source: i32 = l.next().unwrap().parse().unwrap();
                let dest: i32 = l.next().unwrap().parse().unwrap();
                let cost: i32 = l.next().unwrap().parse().unwrap();
                result.add_edge(source, dest, cost);
            }
        }

        Ok(result)
    }

    pub fn new_from_smart_file(filename: String) -> Result<Self, Error> {
        let file_content = fs::read_to_string(filename)?;

        let mut result = Self {
            source_vertices: HashMap::new(),
            destination_vertices: HashMap::new(),
            edge_cost: HashMap::new(),
        };

        for line in file_content.lines() {
            let l = line.to_string();
            let mut l = l.split_whitespace();
            let source: i32 = l.next().unwrap().parse().unwrap();
            let dest: i32 = l.next().unwrap().parse().unwrap();
            let cost: i32 = l.next().unwrap().parse().unwrap();
            if dest == -1 {
                result.add_vertex(source);
            } else {
                result.add_edge(source, dest, cost);
            }
        }

        Ok(result)
    }

    pub fn new_random(vertex_count: i32, edge_count: i32) -> Self {
        let mut result = Self {
            source_vertices: HashMap::new(),
            destination_vertices: HashMap::new(),
            edge_cost: HashMap::new(),
        };

        let mut rng = rand::thread_rng();
        for i in 0..vertex_count {
            result.add_vertex(i);
        }
        for _i in 0..edge_count {
            let mut source = rng.gen_range(0..vertex_count);
            let mut dest = rng.gen_range(0..vertex_count);
            let mut cost = rng.gen_range(0..100);
            while !result.add_edge(source, dest, cost) {
                source = rng.gen_range(0..vertex_count);
                dest = rng.gen_range(0..vertex_count);
                cost = rng.gen_range(0..100); // not really needed..
            }
        }
        return result; 
    }

    pub fn add_vertex(&mut self, vertex: i32) -> bool {
        if let Some(_vec) = self.source_vertices.get(&vertex) {
            return false; // vertex already exists
        } else {
            self.source_vertices.insert(vertex, Vec::new());
            self.destination_vertices.insert(vertex, Vec::new());
            return true;
        }
    }

    pub fn add_edge(&mut self, source_vertex: i32, destination_vertex: i32, cost: i32) -> bool {
        if let Some(_val) = self.edge_cost.get(&(source_vertex, destination_vertex)) {
            return false; // edge already exists
        } else {
            if let Some(vec) = self.source_vertices.get_mut(&source_vertex) {
                vec.push(destination_vertex);
            } else {
                let mut vec: Vec<i32> = Vec::new();
                vec.push(destination_vertex);
                self.source_vertices.insert(source_vertex, vec);
                self.destination_vertices.insert(source_vertex, Vec::new());
            }

            if let Some(vec) = self.destination_vertices.get_mut(&destination_vertex) {
                vec.push(source_vertex);
            } else {
                let mut vec: Vec<i32> = Vec::new();
                vec.push(source_vertex);
                self.destination_vertices.insert(destination_vertex, vec);
                if source_vertex != destination_vertex {
                    self.source_vertices.insert(destination_vertex, Vec::new());
                }
            }
            
            self.edge_cost.insert((source_vertex, destination_vertex), cost);
            return true;
        }
    }

    pub fn remove_vertex(&mut self, vertex: i32) -> bool {
        if let Some(vec) = self.source_vertices.get_mut(&vertex) {
            for dest in vec.clone() {
                let dest_vec = self.destination_vertices.get_mut(&dest).unwrap();
                // unwrap cuz destination_vertex should have that vertex
                dest_vec.remove(dest_vec.iter().position(|v| *v == vertex).unwrap());
                self.edge_cost.remove_entry(&(vertex, dest));
            }
            vec.clear();
        } else {
            return false;
        }
        _ = self.source_vertices.remove_entry(&vertex);

        if let Some(vec) = self.destination_vertices.get_mut(&vertex) {
            for source in  vec.clone() {
                let source_vec = self.source_vertices.get_mut(&source).unwrap();
                // unwrap cuz source_vertex should have that vertex
                // if the vertex was pointing to itself the other  
                // block should have also removed that
                source_vec.remove(source_vec.iter().position(|v| *v == vertex).unwrap());
                if source != vertex {
                    self.edge_cost.remove_entry(&(source, vertex));
                }
            }
            vec.clear();
        } else {
            // vertex doesnt exist, panic or sth..
            return false;
        }
        _ = self.destination_vertices.remove_entry(&vertex);

        return true;
    }

    pub fn remove_edge(&mut self, source: i32, dest: i32) -> Option<i32> {
        if let Some(_val) = self.edge_cost.get(&(source, dest)) {
            let source_vec = self.source_vertices.get_mut(&source).expect("Error finding the source vec");
            source_vec.remove(source_vec.iter().position(|v| *v == dest).expect("Error finding dest vertex in source vec"));
            
            let dest_vec = self.destination_vertices.get_mut(&dest).expect("Error finding the dest vec");
            dest_vec.remove(dest_vec.iter().position(|v| *v == source).expect("Error finding source vertex in dest vec"));
            
            return Some(self.edge_cost.remove(&(source, dest)).unwrap());
        } else {
            return None;
        }
    }

    pub fn get_vertice_size(&self) -> i32 {
        self.source_vertices.len() as i32
    }

    pub fn has_edge(&self, source: i32, dest: i32) -> Option<i32> {
        if let Some(res) = self.edge_cost.get(&(source, dest)) {
            return Some(*res)
        }
        return None
    }
    
    pub fn iterate_source_vertices<'a>(&'a self) -> hash_map::Iter<'a, i32, Vec<i32>> {
        return self.source_vertices.iter()
    }

    pub fn iterate_dest_vertices<'a>(&'a self) -> hash_map::Iter<'a, i32, Vec<i32>> {
        return self.destination_vertices.iter()
    }
    
    pub fn in_degree_of_vertex(&self, vertex: i32) -> Option<i32> {
        if let Some(res) = self.destination_vertices.get(&vertex) {
            return Some(res.len() as i32);
        } else {
            return None;
        }
    }

    pub fn out_degree_of_vertex(&self, vertex: i32) -> Option<i32> {
        if let Some(res) = self.source_vertices.get(&vertex) {
            return Some(res.len() as i32);
        } else {
            return None;
        }
    }

    pub fn inbound_edges_for_vertex<'a>(&'a self, vertex: i32) -> Option<std::slice::Iter<'a, i32>> {
        if let Some(res) = self.destination_vertices.get(&vertex) {
            return Some(res.iter());
        } else {
            return None;
        }
    }

    pub fn outbound_edges_for_vertex<'a>(&'a self, vertex: i32) -> Option<std::slice::Iter<'a, i32>> {
        if let Some(res) = self.source_vertices.get(&vertex) {
            return Some(res.iter());
        } else {
            return None;
        }
    }

    pub fn get_edge_cost(&self, source: i32, dest: i32) -> Option<i32> {
        if let Some(res) = self.edge_cost.get(&(source, dest)) {
            return Some(*res);
        } else {
            return None;
        }
    }

    pub fn set_edge_cost(&mut self, source: i32, dest: i32, cost: i32) -> bool {
        if let hash_map::Entry::Occupied(mut entry) = self.edge_cost.entry((source, dest)) {
            entry.insert(cost);
            return true;
        } else {
            return false;
        }
    }

    pub fn write_graph_to_file_dumb(&self, filename: String) -> Result<bool, Error> {
        let mut f = match File::create(filename) {
            Ok(val) => val,
            Err(err) => return Err(err),
        };

        if let Some(max_key) = self.source_vertices.keys().max() {
            _ = f.write_all(((max_key + 1).to_string() + " " + &self.edge_cost.len().to_string() + "\n").as_bytes());
            for edge in &self.edge_cost {
                _ = f.write_all( (edge.0.0.to_string() + " " + &edge.0.1.to_string() + " " + &edge.1.to_string() + "\n").as_bytes())
            }
            return Ok(true);
        } else {
            return Ok(false);
            // crappy but i have no idea how errors work :)
        }
    }

    pub fn write_graph_to_file_smart(&self, filename: String) -> Result<bool, Error> {
        let mut f = match File::create(filename) {
            Ok(val) => val,
            Err(err) => return Err(err),
        };

        if self.source_vertices.len() == 0 {
            return Ok(false);
        }

        for val in &self.source_vertices {
            if val.1.is_empty() {
                _ = f.write_all((val.0.to_string() + " -1 0\n").as_bytes());
            } else {
                for dest in val.1 {
                    let cost = self.edge_cost.get(&(*val.0, *dest)).unwrap();
                    _ = f.write_all((val.0.to_string() + " " + &dest.to_string() + " " + &cost.to_string() + "\n").as_bytes());
                }
            }
        }
        return Ok(true);
    }

    pub fn print_graph(&self) {
        println!("Source -> Dest");
        for source in &self.source_vertices {
            print!("{}: ", source.0);
            for dest in source.1 {
                print!("{}, ", dest);
            }
            println!("");
        }

        println!("Dest -> Source");
        for dest in &self.destination_vertices {
            print!("{}: ", dest.0);
            for source in dest.1 {
                print!("{}, ", source);
            }
            println!("");
        }

        println!("Edges");
        for edge in &self.edge_cost {
            println!("{} -> {}: {}", edge.0.0, edge.0.1, edge.1);
        }
    }
}