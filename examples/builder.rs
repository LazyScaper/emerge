use csv::ReaderBuilder;
use emerge::graph::{default_window_conf, render_graph, Graph};
use serde::Deserialize;
use std::collections::HashSet;
use std::fs::File;
use std::path::Path;

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
pub struct CountryData {
    name: String,
    first_letter: char,
    last_letter: char,
}

pub fn country_chain_finder() -> Graph {
    print!("Running country name chain finder");

    let file = File::open(Path::new("examples/resources/countries.csv"));

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

    let mut graph = Graph::new();
    let mut used_names = HashSet::new();

    for country_i in starting_candidates.iter() {
        for country_j in starting_candidates.iter() {
            if country_i.first_letter.to_ascii_lowercase()
                == country_j.last_letter.to_ascii_lowercase()
            {
                let country_1_name = &country_i.name;
                let country_2_name = &country_j.name;
                graph.add_directed_edge(country_2_name, country_1_name);

                if !used_names.contains(&country_i.name) {
                    graph.add_node(country_1_name);
                }

                if !used_names.contains(&country_j.name) {
                    graph.add_node(country_2_name);
                }

                used_names.insert(country_1_name.clone());
                used_names.insert(country_2_name.clone());
            }
        }
    }

    graph
}

#[macroquad::main(default_window_conf)]
async fn main() {
    let graph = country_chain_finder();
    render_graph(graph).await;
}
