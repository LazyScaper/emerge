use csv::ReaderBuilder;
use macroquad::prelude::{screen_height, screen_width};
use random::Rng;
use serde::Deserialize;
use std::collections::HashSet;
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
pub struct Node {
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
    pub(crate) name: String,
    pub(crate) first_letter: char,
    pub(crate) last_letter: char,
}

#[derive(Debug, Deserialize)]
pub struct PhysicsData {
    pub(crate) mass: Mass,
    pub(crate) velocity: Velocity,
    pub(crate) force: Force,
    pub(crate) position: Position,
    pub(crate) size: Size,
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
