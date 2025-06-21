use crate::graph::{Edge, Position, Size};
use hecs::World;
use macroquad::color::{Color, BLACK, RED, WHITE};
use macroquad::math::Vec2;
use macroquad::prelude::{clear_background, draw_circle, draw_line};
use macroquad::text::{draw_text, get_text_center};
use std::collections::HashMap;

pub fn render(world: &mut World) {
    clear_background(RED);

    for (_id, (position, size, label)) in &mut world.query::<(&mut Position, &Size, &String)>() {
        draw_circle(position.x, position.y, size.radius, BLACK);
        let label = &label;
        let center_of_text = get_text_center(label, Option::None, 20, 1.0, 0.0);
        draw_text(
            label,
            position.x - center_of_text.x,
            position.y - center_of_text.y / 2.0,
            20.0,
            WHITE,
        );
    }

    let node_data: HashMap<usize, Position> = world
        .query::<(&Position, &usize)>()
        .iter()
        .map(|(e, (pos, &node_id))| (node_id, pos.clone()))
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
            if let Some(source_node_position) = node_data.get(&edge_source_node_id) {
                if let Some(destination_node_position) = node_data.get(&edge_destination_node_id) {
                    draw_arrow_line(
                        Vec2 {
                            x: source_node_position.x,
                            y: source_node_position.y,
                        },
                        Vec2 {
                            x: destination_node_position.x,
                            y: destination_node_position.y,
                        },
                        15.0,
                        15.0,
                        WHITE,
                        3.0,
                    )
                }
            }
        };
    }
}

fn render_arrow(
    color: Color,
    thickness: f32,
    arrow_start: Vec2,
    arrow_end: Vec2,
    arrowhead_left: Vec2,
    arrowhead_right: Vec2,
) {
    // Draw main line
    draw_line(
        arrow_start.x,
        arrow_start.y,
        arrow_end.x,
        arrow_end.y,
        thickness,
        color,
    );
    // draw left side of arrow head
    draw_line(
        arrow_end.x,
        arrow_end.y,
        arrowhead_left.x,
        arrowhead_left.y,
        thickness,
        color,
    );
    // draw right side of arrow head
    draw_line(
        arrow_end.x,
        arrow_end.y,
        arrowhead_right.x,
        arrowhead_right.y,
        thickness,
        color,
    );
}

fn calculate_arrow_positions(
    start_pos: Vec2,
    end_pos: Vec2,
    start_node_radius: f32,
    end_node_radius: f32,
) -> (Vec2, Vec2, Vec2, Vec2) {
    // Calculate direction vector
    let direction = (end_pos - start_pos).normalize();

    // Calculate arrow start and end points (edge of circles)
    let arrow_start_pos = start_pos + direction * start_node_radius;
    let arrow_end_pos = end_pos - direction * end_node_radius;

    // Draw arrowhead
    let arrowhead_size = 15.0;
    // To find the perpendicular vector [x,y] to [i,j] we need the dot product to be zero
    // Thus u dot v = 0 => xi + yj = 0. From inspection we can see x = j and y = -i would suffice.
    let perpendicular = Vec2::new(direction.y, -direction.x);

    let arrowhead_left_end_pos =
        arrow_end_pos - direction * arrowhead_size + perpendicular * (arrowhead_size * 0.5);
    let arrowhead_right_end_pos =
        arrow_end_pos - direction * arrowhead_size - perpendicular * (arrowhead_size * 0.5);
    (
        arrow_start_pos,
        arrow_end_pos,
        arrowhead_left_end_pos,
        arrowhead_right_end_pos,
    )
}

fn draw_arrow_line(
    start_pos: Vec2,
    end_pos: Vec2,
    start_radius: f32,
    end_radius: f32,
    color: Color,
    thickness: f32,
) {
    let (arrow_start_pos, arrow_end_pos, arrowhead_left_end_pos, arrowhead_right_end_pos) =
        calculate_arrow_positions(start_pos, end_pos, start_radius, end_radius);

    render_arrow(
        color,
        thickness,
        arrow_start_pos,
        arrow_end_pos,
        arrowhead_left_end_pos,
        arrowhead_right_end_pos,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calculate_arrow_positions() {
        let (start_pos, end_pos, start_node_radius, end_node_radius) =
            (Vec2 { x: 0.0, y: 0.0 }, Vec2 { x: 100.0, y: 0.0 }, 2.0, 2.0);

        let (arrow_start_pos, arrow_end_pos, arrowhead_left_end_pos, arrowhead_right_end_pos) =
            calculate_arrow_positions(start_pos, end_pos, start_node_radius, end_node_radius);

        assert_eq!(arrow_start_pos, Vec2 { x: 2.0, y: 0.0 });
        assert_eq!(arrow_end_pos, Vec2 { x: 98.0, y: 0.0 });
        assert_eq!(arrowhead_left_end_pos, Vec2 { x: 83.0, y: -7.5 });
        assert_eq!(arrowhead_right_end_pos, Vec2 { x: 83.0, y: 7.5 });
    }
}
