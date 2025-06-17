use csv::ReaderBuilder;
use macroquad::prelude::{screen_height, screen_width};
use random::Rng;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Mass {
    pub(crate) mass: f32,
}

#[derive(Debug, Deserialize)]
pub struct Velocity {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

#[derive(Debug, Deserialize)]
pub struct Force {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Position {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

#[derive(Debug, Deserialize)]
pub struct Size {
    pub(crate) radius: f32,
}

#[derive(Debug, Deserialize)]
struct Country {
    #[serde(rename = "Country code")]
    country_code: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Official name")]
    official_name: String,
    #[serde(rename = "Citizen names")]
    citizens_name: String,
}

#[derive(Debug, Deserialize)]
pub struct NodeId {
    pub(crate) id: usize,
}

#[derive(Debug, Deserialize)]
pub struct CountryNode {
    pub(crate) id: NodeId,
    pub(crate) country_data: CountryData,
    pub(crate) physics_data: PhysicsData,
    pub(crate) outgoing_edges: HashSet<usize>,
    pub(crate) incoming_edges: HashSet<usize>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Edge {
    pub(crate) source_node_id: usize,
    pub(crate) destination_node_id: usize,
}

#[derive(Debug, Deserialize)]
pub struct CountryData {
    name: String,
    first_letter: char,
    last_letter: char,
}

#[derive(Debug, Deserialize)]
pub struct PhysicsData {
    pub(crate) mass: Mass,
    pub(crate) velocity: Velocity,
    pub(crate) force: Force,
    pub(crate) position: Position,
    pub(crate) size: Size,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Graph {
    pub(crate) nodes: Vec<CountryNode>,
    pub(crate) node_lookup: HashMap<String, usize>,
}

impl PhysicsData {
    pub fn init() -> Self {
        let mut rng = random::rng();

        Self {
            mass: Mass { mass: 0.0 },
            velocity: Velocity { x: 0.0, y: 0.0 },
            force: Force { x: 0.0, y: 0.0 },
            position: Position {
                x: rng.random_range(0.0..screen_width()),
                y: rng.random_range(0.0..screen_height()),
            },
            size: Size { radius: 15.0 },
        }
    }
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            node_lookup: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, name: String, node_data: CountryData) {
        let id = self.nodes.len();

        self.node_lookup.insert(name, id);
        self.nodes.push(CountryNode {
            id: NodeId { id },
            country_data: node_data,
            physics_data: PhysicsData::init(),
            outgoing_edges: HashSet::new(),
            incoming_edges: HashSet::new(),
        });
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        if from == to {
            return;
        }

        self.nodes.get_mut(from).unwrap().outgoing_edges.insert(to);
        self.nodes.get_mut(to).unwrap().incoming_edges.insert(from);
    }
    pub fn add_edge_by_name(&mut self, from_name: &str, to_name: &str) {
        if from_name.eq(to_name) {
            return;
        }

        if let (Some(&from_id), Some(&to_id)) = (
            self.node_lookup.get(from_name),
            self.node_lookup.get(to_name),
        ) {
            self.add_edge(from_id, to_id);
        }
    }

    pub fn get_all_edges(&self) -> Vec<Edge> {
        let mut edges = Vec::new();

        for source_node in self.nodes.iter() {
            for &destination_node_id in &source_node.outgoing_edges {
                edges.push(Edge {
                    source_node_id: source_node.id.id,
                    destination_node_id,
                });
            }
        }

        edges
    }
}

pub fn country_chain_finder() {
    print!("Running country name chain finder");

    let file = File::open(Path::new("src/resources/countries.csv"));

    let file = match file {
        Ok(file) => file,
        Err(e) => panic!("There has been an issue opening the file: {:?}", e),
    };

    let mut csv_reader = ReaderBuilder::new().from_reader(file);

    let mut starting_candidates = Vec::new();

    for country in csv_reader.deserialize::<Country>() {
        let country = match country {
            Ok(c) => c,
            Err(e) => panic!("There was an issue parsing the CSV file: {:?}", e),
        };

        let first_letter = country.name.chars().next().unwrap();
        let last_letter = country.name.chars().last().unwrap();

        starting_candidates.push(CountryData {
            name: country.name,
            first_letter,
            last_letter,
        });

        // starting_candidates.clo
    }

    for starting_candidate in starting_candidates {
        println!("Starting candidate {:?}", starting_candidate.name);

        println!("{:?}", starting_candidate);
    }
}

pub fn build_graph() -> Graph {
    let mut graph = Graph::new();

    graph.add_node(
        "Albania".to_string(),
        CountryData {
            name: "Albania".to_string(),
            last_letter: 'a',
            first_letter: 'a',
        },
    );
    graph.add_node(
        "Cambodia".to_string(),
        CountryData {
            name: "Cambodia".to_string(),
            last_letter: 'c',
            first_letter: 'a',
        },
    );
    graph.add_node(
        "Cameroon".to_string(),
        CountryData {
            name: "Cameroon".to_string(),
            last_letter: 'c',
            first_letter: 'n',
        },
    );
    graph.add_node(
        "Nigeria".to_string(),
        CountryData {
            name: "Nigeria".to_string(),
            last_letter: 'n',
            first_letter: 'a',
        },
    );
    graph.add_edge_by_name("Cambodia", "Albania");
    graph.add_edge_by_name("Cameroon", "Nigeria");
    graph.add_edge_by_name("Nigeria", "Albania");

    graph
}
