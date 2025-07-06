use crate::physics::physics_update;
use crate::renderer;
use crate::renderer::{render, ScrollableView};
use hecs::World;
use macroquad::color::BLACK;
use macroquad::prelude::next_frame;
use macroquad::prelude::Conf;
use macroquad::window::{screen_height, screen_width};
use std::collections::{HashMap, HashSet};
use std::f32::consts::PI;

#[derive(Debug)]
pub(crate) struct Velocity {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

#[derive(Debug)]
pub(crate) struct Force {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

#[derive(Debug, Clone)]
pub(crate) struct Position {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

#[derive(Debug)]
pub(crate) struct Size {
    pub(crate) radius: f32,
}

#[derive(Debug)]
pub(crate) struct Node {
    pub(crate) id: usize,
    pub(crate) label: String,
    pub(crate) physics_data: PhysicsData,
    pub(crate) outgoing_directed_edges: HashSet<usize>,
    pub(crate) incoming_directed_edges: HashSet<usize>,
    pub(crate) outgoing_undirected_edges: HashSet<usize>,
    pub(crate) incoming_undirected_edges: HashSet<usize>,
}

#[derive(Debug, Clone)]
pub(crate) struct Edge {
    pub(crate) source_node_id: usize,
    pub(crate) destination_node_id: usize,
    pub(crate) is_directed: bool,
}

#[derive(Debug)]
pub(crate) struct PhysicsData {
    pub(crate) velocity: Velocity,
    pub(crate) force: Force,
    pub(crate) size: Size,
}

impl PhysicsData {
    pub fn init() -> Self {
        Self {
            velocity: Velocity { x: 0.0, y: 0.0 },
            force: Force { x: 0.0, y: 0.0 },
            size: Size { radius: 15.0 },
        }
    }
}

#[derive(Debug)]
pub struct Graph {
    pub(crate) nodes: Vec<Node>,
    pub(crate) node_lookup: HashMap<String, usize>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            node_lookup: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, label: &str) {
        let id = self.nodes.len();

        self.node_lookup.insert(label.to_string(), id);
        self.nodes.push(Node {
            id,
            label: label.to_string(),
            physics_data: PhysicsData::init(),
            outgoing_directed_edges: HashSet::new(),
            incoming_directed_edges: HashSet::new(),
            outgoing_undirected_edges: HashSet::new(),
            incoming_undirected_edges: HashSet::new(),
        });
    }

    pub fn add_directed_edge(&mut self, from: &str, to: &str) {
        if from.eq(to) {
            return;
        }

        if let (Some(&from_id), Some(&to_id)) =
            (self.node_lookup.get(from), self.node_lookup.get(to))
        {
            self.nodes
                .get_mut(from_id)
                .unwrap()
                .outgoing_directed_edges
                .insert(to_id);
            self.nodes
                .get_mut(to_id)
                .unwrap()
                .incoming_directed_edges
                .insert(from_id);
        }
    }

    pub fn add_undirected_edge(&mut self, from: &str, to: &str) {
        if from.eq(to) {
            return;
        }

        if let (Some(&from_id), Some(&to_id)) =
            (self.node_lookup.get(from), self.node_lookup.get(to))
        {
            self.nodes
                .get_mut(from_id)
                .unwrap()
                .outgoing_undirected_edges
                .insert(to_id);
            self.nodes
                .get_mut(to_id)
                .unwrap()
                .incoming_undirected_edges
                .insert(from_id);
        }
    }

    fn get_node_by_name(&self, name: &str) -> Option<&Node> {
        self.node_lookup
            .get(name)
            .and_then(|&index| self.nodes.get(index))
    }

    fn get_all_edges(&self) -> Vec<Edge> {
        let mut edges = Vec::new();

        for source_node in self.nodes.iter() {
            for &destination_node_id in &source_node.outgoing_directed_edges {
                edges.push(Edge {
                    source_node_id: source_node.id,
                    destination_node_id: destination_node_id,
                    is_directed: true,
                });
            }
        }

        for source_node in self.nodes.iter() {
            for &destination_node_id in &source_node.outgoing_undirected_edges {
                edges.push(Edge {
                    source_node_id: source_node.id,
                    destination_node_id: destination_node_id,
                    is_directed: false,
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
    let mut world = spawn_initial(graph);

    loop {
        render(&mut world);

        physics_update(&mut world);

        renderer::view_port_update(&mut world);

        next_frame().await
    }
}

fn spawn_initial(graph: Graph) -> World {
    let mut world = World::new();
    let view = ScrollableView::new();

    world.spawn((view,));

    let all_edges = graph.get_all_edges();
    let node_count = graph.nodes.len();

    for (index, node) in graph.nodes.into_iter().enumerate() {
        let angle = 2.0 * PI * index as f32 / node_count as f32;
        let x = screen_width() / 2.0 + 600.0 * angle.cos();
        let y = screen_height() / 2.0 + 300.0 * angle.sin();

        let renderable_node = (
            node.id,
            node.physics_data.velocity,
            node.physics_data.force,
            Position { x, y },
            node.physics_data.size,
            node.label,
            BLACK,
        );

        world.spawn(renderable_node);
    }

    for edge in all_edges {
        world.spawn((edge,));
    }

    world
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_add_node() {
        let mut graph = Graph::new();

        graph.add_node("A");
        graph.add_node("B");
        graph.add_node("C");
        graph.add_node("D");

        assert_eq!(graph.nodes.len(), 4);
    }

    #[test]
    fn should_maintain_incoming_and_outgoing_edges() {
        let mut graph = Graph::new();

        graph.add_node("A");
        graph.add_node("B");
        graph.add_directed_edge("A", "B");

        let node_a = graph.get_node_by_name("A").unwrap();
        let node_b = graph.get_node_by_name("B").unwrap();

        assert!(node_a.outgoing_directed_edges.contains(&node_b.id));
        assert!(node_b.incoming_directed_edges.contains(&node_a.id));
    }
}
