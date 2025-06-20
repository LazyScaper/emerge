use crate::builder::{CountryData, Edge, Node, NodeId, PhysicsData};
use crate::physics::{physics_update, simulate_time_step};
use crate::renderer::render;
use hecs::World;
use macroquad::color::BLACK;
use macroquad::prelude::next_frame;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Deserialize)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub(crate) node_lookup: HashMap<String, usize>,
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
        self.nodes.push(Node {
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

pub async fn render_graph(mut world: &mut World, graph: Graph) {
    let all_edges = graph.get_all_edges();
    for node in graph.nodes {
        let renderable_node = (
            node.id,
            node.physics_data.velocity,
            node.physics_data.force,
            node.physics_data.mass,
            node.physics_data.position,
            node.physics_data.size,
            node.country_data,
            BLACK,
        );

        world.spawn(renderable_node);
    }

    for edge in all_edges {
        world.spawn((edge,));
    }

    loop {
        render(&mut world);

        // physics calc, update forces
        // plug into equations of motion to calc velocity
        physics_update(&mut world);

        // simulate small time step, update positions
        simulate_time_step(&mut world);

        next_frame().await
    }
}
