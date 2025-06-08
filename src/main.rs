use crate::graph_renderer::render;
use hecs::World;
use macroquad::prelude::*;

mod graph_builder;
mod graph_renderer;

fn window_conf() -> Conf {
    Conf {
        window_title: "Circle Resizing Examples".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: false,
        ..Default::default()
    }
}

fn physics_update(world: World) {}

#[macroquad::main(window_conf)]
async fn main() {
    let mut world = World::new();

    // let node = (
    //     Mass { mass: 1.0 },
    //     Position { x: 100.0, y: 100.0 },
    //     Size { radius: 15.0 },
    //     NodeColor { color: BLACK },
    // );
    //
    // world.spawn(node);

    loop {
        render(&mut world);

        next_frame().await
    }
}
