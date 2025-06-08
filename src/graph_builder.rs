use csv::ReaderBuilder;
use serde::Deserialize;
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
struct CountryNode {
    name: String,
    first_letter: char,
    last_letter: char,
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

        starting_candidates.push(CountryNode {
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
