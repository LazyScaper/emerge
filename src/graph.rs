use crate::physics::physics_update;
use crate::renderer::render;
use hecs::World;
use macroquad::color::BLACK;
use macroquad::prelude::Conf;
use macroquad::prelude::{next_frame, screen_height, screen_width};
use random::Rng;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};

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
pub struct Node {
    pub(crate) id: usize,
    pub(crate) label: String,
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
pub struct PhysicsData {
    pub(crate) velocity: Velocity,
    pub(crate) force: Force,
    pub(crate) position: Position,
    pub(crate) size: Size,
}

impl PhysicsData {
    pub fn init() -> Self {
        let mut rng = random::rng();

        Self {
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

    pub fn add_node(&mut self, label: String) {
        let id = self.nodes.len();

        self.node_lookup.insert(label.clone(), id);
        self.nodes.push(Node {
            id,
            label,
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
                    source_node_id: source_node.id,
                    destination_node_id,
                });
            }
        }

        edges
    }
}

pub fn default_window_conf() -> Conf {
    Conf {
        window_title: "Emerge - Graph".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: false,
        ..Default::default()
    }
}

pub async fn render_graph(graph: Graph) {
    let mut world = World::new();

    let all_edges = graph.get_all_edges();
    for node in graph.nodes {
        let renderable_node = (
            node.id,
            node.physics_data.velocity,
            node.physics_data.force,
            node.physics_data.position,
            node.physics_data.size,
            node.label,
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

        next_frame().await
    }
}
