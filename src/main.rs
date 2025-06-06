use macroquad::prelude::*;
mod chain_builders;

trait DrawableNode {
    fn render(&self) -> ();
}

struct Node {
    x_pos: f32,
    y_pos: f32,
    radius: f32,
    color: Color,
}

impl DrawableNode for Node {
    fn render(&self) {
        draw_circle(self.x_pos, self.y_pos, self.radius, self.color);
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        physics_update();
        render();
        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Circle Resizing Examples".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: false,
        ..Default::default()
    }
}

fn render() {
    clear_background(RED);

    let node = Node {
        x_pos: screen_width() / 2f32,
        y_pos: screen_height() / 2f32,
        radius: 15.0,
        color: WHITE,
    };
    let node1 = Node {
        x_pos: screen_width() / 2f32 + 45f32,
        y_pos: screen_height() / 2f32 + 45f32,
        radius: 15.0,
        color: WHITE,
    };

    node.render();
    node1.render();

    draw_arrow_line(
        Vec2 {
            x: node.x_pos,
            y: node.y_pos,
        },
        Vec2 {
            x: node1.x_pos,
            y: node1.y_pos,
        },
        node.radius,
        node1.radius,
        WHITE,
        3.0,
    );
}

fn draw_arrow_line(
    start_pos: Vec2,
    end_pos: Vec2,
    start_radius: f32,
    end_radius: f32,
    color: Color,
    thickness: f32,
) {
    // Calculate direction vector
    let direction = (end_pos - start_pos).normalize();

    // Calculate arrow start and end points (edge of circles)
    let arrow_start = start_pos + direction * start_radius;
    let arrow_end = end_pos - direction * end_radius;

    // Draw main line
    draw_line(
        arrow_start.x,
        arrow_start.y,
        arrow_end.x,
        arrow_end.y,
        thickness,
        color,
    );

    // Draw arrowhead
    let arrowhead_size = 15.0;
    let perpendicular = Vec2::new(-direction.y, direction.x);

    let arrowhead_left =
        arrow_end - direction * arrowhead_size + perpendicular * (arrowhead_size * 0.5);
    let arrowhead_right =
        arrow_end - direction * arrowhead_size - perpendicular * (arrowhead_size * 0.5);

    draw_line(
        arrow_end.x,
        arrow_end.y,
        arrowhead_left.x,
        arrowhead_left.y,
        thickness,
        color,
    );
    draw_line(
        arrow_end.x,
        arrow_end.y,
        arrowhead_right.x,
        arrowhead_right.y,
        thickness,
        color,
    );
}

fn physics_update() {}
