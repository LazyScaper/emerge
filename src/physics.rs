use crate::graph::{Edge, Force, Position, Velocity};
use hecs::World;
use std::collections::HashMap;

const TIME_STEP: f32 = 0.1f32;
const SPRING_CONSTANT: f32 = 1f32;
const SPRING_RESTING_LENGTH: f32 = 100f32;
const ELECTROSTATIC_CONSTANT: f32 = 20000f32;

pub fn physics_update(world: &mut World) {
    let node_data = node_positions_by_id(world);

    apply_attractive_forces(world, &node_data);

    apply_repulsive_forces(world, &node_data);

    simulate_time_step(world);

    clear_all_forces(world)
}

fn apply_repulsive_forces(world: &mut World, node_data: &HashMap<usize, Position>) {
    for (&first_node_id, first_node_position) in node_data {
        for (&second_node_id, second_node_position) in node_data {
            if first_node_id != second_node_id
                && is_in_range(first_node_position, second_node_position)
            {
                let force = calculate_electrostatic_forces_between_nodes(
                    &first_node_position,
                    &second_node_position,
                );
                apply_force_to_node(
                    world,
                    first_node_id,
                    Force {
                        x: -force.x,
                        y: -force.y,
                    },
                );
            }
        }
    }
}

fn clear_all_forces(world: &mut World) {
    let node_data: HashMap<usize, Position> = node_positions_by_id(world);

    for (&id, _) in &node_data {
        match world
            .query::<(&mut Force, &usize)>()
            .iter()
            .find(|(_entity, (_force, &node_id))| id == node_id)
        {
            Some((_found_node, (force, _node_id))) => {
                force.x = 0.0;
                force.y = 0.0;
            }
            _ => {
                panic!("Could not find the node to apply a force to, this really should not happen")
            }
        }
    }
}

fn apply_attractive_forces(world: &mut World, node_data: &HashMap<usize, Position>) {
    let edge_data = edge_by_id(world);

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
                    let attractive_force_between_nodes = calculate_spring_forces_between_nodes(
                        destination_node_position,
                        source_node_position,
                    );

                    apply_force_to_node(
                        world,
                        edge_source_node_id,
                        Force {
                            x: attractive_force_between_nodes.x,
                            y: attractive_force_between_nodes.y,
                        },
                    );
                    apply_force_to_node(
                        world,
                        edge_destination_node_id,
                        Force {
                            x: -attractive_force_between_nodes.x,
                            y: -attractive_force_between_nodes.y,
                        },
                    );
                }
                _ => {}
            }
        };
    }
}

fn apply_force_to_node(world: &mut World, node_id_to_match: usize, force_to_apply: Force) {
    match world
        .query::<(&mut Force, &usize)>()
        .iter()
        .find(|(_entity, (_force, &node_id))| node_id_to_match == node_id)
    {
        Some((_found_node, (force, _node_id))) => {
            force.x += force_to_apply.x;
            force.y += force_to_apply.y;
        }
        _ => {
            panic!("Could not find the node to apply a force to, this really should not happen")
        }
    }
}

fn calculate_spring_forces_between_nodes(
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

fn calculate_electrostatic_forces_between_nodes(
    source_node_position: &Position,
    destination_node_position: &Position,
) -> Force {
    let dx = destination_node_position.x - source_node_position.x;
    let dy = destination_node_position.y - source_node_position.y;

    let distance_between_nodes = (dx * dx + dy * dy).sqrt();

    Force {
        x: ELECTROSTATIC_CONSTANT / distance_between_nodes.powi(2) * (dx / distance_between_nodes),
        y: ELECTROSTATIC_CONSTANT / distance_between_nodes.powi(2) * (dy / distance_between_nodes),
    }
}

fn simulate_time_step(world: &mut World) {
    for (_id, (position, velocity, force)) in
        world.query_mut::<(&mut Position, &mut Velocity, &mut Force)>()
    {
        position.x += velocity.x * TIME_STEP + 0.5 * force.x * TIME_STEP.powi(2);
        position.y += velocity.y * TIME_STEP + 0.5 * force.y * TIME_STEP.powi(2);
    }
}

pub(crate) fn node_positions_by_id(world: &mut World) -> HashMap<usize, Position> {
    world
        .query::<(&Position, &usize)>()
        .iter()
        .map(|(_, (pos, &node_id))| (node_id, pos.clone()))
        .collect()
}

pub(crate) fn edge_by_id(world: &mut World) -> HashMap<usize, Edge> {
    world
        .query::<&Edge>()
        .iter()
        .map(|(_, edge)| (edge.source_node_id, edge.clone()))
        .collect()
}

fn is_in_range(p0: &Position, p1: &Position) -> bool {
    ((p0.x - p1.x).powi(2) + (p0.y - p1.y).powi(2)).sqrt() < 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! spring_forces_tests {
        ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (first_position, second_position, expected_force) = $value;
            assert_eq!(expected_force.x, calculate_spring_forces_between_nodes(&first_position, &second_position).x);
            assert_eq!(expected_force.y, calculate_spring_forces_between_nodes(&first_position, &second_position).y);
        }
    )*
    }}

    macro_rules! electrostatic_forces_tests {
        ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (first_position, second_position, expected_force) = $value;
            assert_eq!(expected_force.x, calculate_electrostatic_forces_between_nodes(&first_position, &second_position).x);
            assert_eq!(expected_force.y, calculate_electrostatic_forces_between_nodes(&first_position, &second_position).y);
        }
    )*
    }}

    spring_forces_tests! {
            spring_forces_1: (Position { x: 0.0, y: 0.0 }, Position { x: 300.0, y: 0.0 }, Force{ x: -200.0, y: 0.0}),
            spring_forces_2: (Position { x: 0.0, y: 300.0 }, Position { x: 0.0, y: 0.0 }, Force{ x: 0.0, y: 200.0}),
            spring_forces_3: (Position { x: 0.0, y: 0.0 }, Position { x: 300.0, y: 300.0 }, Force{ x: -229.28932, y: -229.28932}),
    }

    electrostatic_forces_tests! {
            electrostatic_forces_1: (Position { x: 0.0, y: 0.0 }, Position { x: 10.0, y: 0.0 }, Force{ x: 200.0, y: 0.0}),
    }
}
