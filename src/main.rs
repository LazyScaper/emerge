use crate::renderer::render;
use hecs::World;
use macroquad::prelude::*;
use std::fmt::Debug;
use std::hash::Hash;

mod builder;
mod graph;
mod physics;
mod renderer;

use crate::graph::build_graph;
use crate::physics::{physics_update, simulate_time_step};
use random::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Emerge - Graph".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: false,
        ..Default::default()
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
