use crate::graph_renderer::render;
use hecs::World;
use macroquad::prelude::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

mod graph_builder;
mod graph_renderer;
use crate::graph_builder::*;
use random::prelude::*;

const TIME_STEP: f32 = 0.5f32;
const SPRING_CONSTANT: f32 = 0.5f32;
const SPRING_RESTING_LENGTH: f32 = 100f32;

fn window_conf() -> Conf {
    Conf {
        window_title: "Circle Resizing Examples".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: false,
        ..Default::default()
    }
}

fn physics_update(world: &mut World) {
    let node_data: HashMap<usize, Position> = world
        .query::<(&Position, &NodeId)>()
        .iter()
        .map(|(e, (pos, node_id))| (node_id.id, pos.clone()))
        .collect();

    let edge_data: HashMap<usize, Edge> = world
        .query::<(&Edge)>()
        .iter()
        .map(|(e, edge)| (edge.source_node_id, edge.clone()))
        .collect();

    for (_, edge) in edge_data {
        let edge_source_node_id = edge.source_node_id;
        let edge_destination_node_id = edge.destination_node_id;

        if node_data.contains_key(&edge_source_node_id)
            && node_data.contains_key(&edge_destination_node_id)
        {
            match (
                node_data.get(&edge_source_node_id),
                node_data.get(&edge_destination_node_id),
            ) {
                (Some(source_node_position), Some(destination_node_position)) => {
                    apply_forces_between_nodes(
                        world,
                        edge_source_node_id,
                        destination_node_position,
                        source_node_position,
                    );
                    apply_forces_between_nodes(
                        world,
                        edge_destination_node_id,
                        source_node_position,
                        destination_node_position,
                    );
                }
                _ => {}
            }
        };
    }
}

fn apply_forces_between_nodes(
    world: &mut World,
    node_id_to_match: usize,
    source_node_position: &Position,
    destination_node_position: &Position,
) {
    match world
        .query::<(&mut Force, &NodeId)>()
        .iter()
        .find(|(_entity, (_force, node_id))| node_id_to_match == node_id.id)
    {
        Some((_found_node, (mut force, _node_id))) => {
            let dx = destination_node_position.x - source_node_position.x;
            let dy = destination_node_position.y - source_node_position.y;

            let current_length = (dx * dx + dy * dy).sqrt();
            let displacement_from_rest = current_length - SPRING_RESTING_LENGTH;

            force.x = -SPRING_CONSTANT * displacement_from_rest * (dx / current_length);
            force.y = -SPRING_CONSTANT * displacement_from_rest * (dy / current_length);
        }
        None => {}
    }
}

fn simulate_time_step(world: &mut World) {
    for (_id, (position, velocity, force, mass)) in
        world.query_mut::<(&mut Position, &mut Velocity, &mut Force, &Mass)>()
    {
        position.x += velocity.x * TIME_STEP + 0.5 * force.x * TIME_STEP.powi(2);
        position.y += velocity.y * TIME_STEP + 0.5 * force.y * TIME_STEP.powi(2);
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut world = World::new();
    let graph = build_graph();

    let all_edges = graph.get_all_edges();
    for node in graph.nodes {
        let renderable_node = (
            node.id,
            node.physics_data.velocity,
            node.physics_data.force,
            node.physics_data.mass,
            node.physics_data.position,
            node.physics_data.size,
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
