use crate::graph::{Edge, Force, Mass, NodeId, Position, Velocity};
use hecs::World;
use std::collections::HashMap;

const TIME_STEP: f32 = 0.5f32;
const SPRING_CONSTANT: f32 = 0.5f32;
const SPRING_RESTING_LENGTH: f32 = 100f32;

pub fn physics_update(world: &mut World) {
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
                    let force_between_nodes = calculate_forces_between_nodes(
                        destination_node_position,
                        source_node_position,
                    );
                    apply_force_to_node(
                        world,
                        edge_source_node_id,
                        Force {
                            x: force_between_nodes.x,
                            y: force_between_nodes.y,
                        },
                    );
                    apply_force_to_node(
                        world,
                        edge_destination_node_id,
                        Force {
                            x: -force_between_nodes.x,
                            y: -force_between_nodes.y,
                        },
                    );
                }
                _ => {}
            }
        };
    }
}

fn apply_force_to_node(world: &mut World, node_id_to_match: usize, nodes: Force) {
    match world
        .query::<(&mut Force, &NodeId)>()
        .iter()
        .find(|(_entity, (_force, node_id))| node_id_to_match == node_id.id)
    {
        Some((_found_node, (force, _node_id))) => {
            *force = nodes;
        }
        _ => {
            panic!("Could not find the node to apply a force to, this really should not happen")
        }
    }
}

fn calculate_forces_between_nodes(
    source_node_position: &Position,
    destination_node_position: &Position,
) -> Force {
    let dx = destination_node_position.x - source_node_position.x;
    let dy = destination_node_position.y - source_node_position.y;

    let current_length = (dx * dx + dy * dy).sqrt();
    let displacement_from_rest = current_length - SPRING_RESTING_LENGTH;

    Force {
        x: -SPRING_CONSTANT * displacement_from_rest * (dx / current_length),
        y: -SPRING_CONSTANT * displacement_from_rest * (dy / current_length),
    }
}

pub fn simulate_time_step(world: &mut World) {
    for (_id, (position, velocity, force, mass)) in
        world.query_mut::<(&mut Position, &mut Velocity, &mut Force, &Mass)>()
    {
        position.x += velocity.x * TIME_STEP + 0.5 * force.x * TIME_STEP.powi(2);
        position.y += velocity.y * TIME_STEP + 0.5 * force.y * TIME_STEP.powi(2);
    }
}
