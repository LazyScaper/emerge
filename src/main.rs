use crate::graph_renderer::render;
use hecs::World;
use macroquad::prelude::*;

mod graph_builder;
mod graph_renderer;
use crate::graph_builder::*;
use random::prelude::*;

const TIME_STEP: f32 = 0.1f32;
const SPRING_CONSTANT: f32 = 0.1f32;

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
    for (id, (velocity, force, mass)) in world.query_mut::<(&mut Velocity, &mut Force, &Mass)>() {
        // todo!()
    }
}

fn simulate_time_step(world: &mut World) {
    for (id, (position, velocity, force, mass)) in
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

    for node in graph.nodes {
        let renderable_node = (
            node.physics_data.velocity,
            node.physics_data.force,
            node.physics_data.mass,
            node.physics_data.position,
            node.physics_data.size,
            node.incoming_edges,
            BLACK,
        );

        world.spawn(renderable_node);
    }

    loop {
        render(&mut world);

        // physics calc, update forces
        physics_update(&mut world);

        // plug into equations of motion to calc velocity

        // simulate small time step, update positions
        simulate_time_step(&mut world);

        next_frame().await
    }
}
