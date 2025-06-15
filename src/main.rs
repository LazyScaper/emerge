use crate::graph_renderer::render;
use hecs::World;
use macroquad::prelude::*;

mod graph_builder;
mod graph_renderer;
use crate::graph_builder::*;
use random::prelude::*;

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
            BLACK
        );

        world.spawn(renderable_node);
    }

    loop {
        render(&mut world);
        // physics calc, update forces
        physics_update(&mut world);

        // plug into equations of motion to calc velocity
        
        // simulate small time step, update positions
        next_frame().await
    }
}
